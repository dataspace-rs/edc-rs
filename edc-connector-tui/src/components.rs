use std::{fmt::Debug, future::Future, sync::Arc};

use crossterm::event::Event;
use futures::{future::BoxFuture, FutureExt};
use ratatui::{layout::Rect, Frame};

use crate::types::nav::Nav;

pub mod assets;
pub mod connectors;
pub mod contract_definitions;
pub mod footer;
pub mod header;
pub mod launch_bar;
pub mod policies;
pub mod resources;
pub mod table;

pub trait StatelessComponent {
    type Props: Send;

    fn view(&mut self, props: &Self::Props, f: &mut Frame, rect: Rect);
}

#[async_trait::async_trait]
pub trait Component {
    type Msg: Send;
    type Props: Send;

    async fn init(&mut self, _props: Self::Props) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        Ok(ComponentReturn::empty())
    }

    fn view(&mut self, f: &mut Frame, rect: Rect);

    async fn update(
        &mut self,
        _message: ComponentMsg<Self::Msg>,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>> {
        Ok(ComponentReturn::empty())
    }

    fn handle_event(
        &mut self,
        _evt: ComponentEvent,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>> {
        Ok(vec![])
    }

    async fn forward_update<'a, F, C>(
        other: &'a mut C,
        msg: ComponentMsg<C::Msg>,
        mapper: F,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>>
    where
        F: Fn(C::Msg) -> Self::Msg + Send + Sync + 'a,
        C: Component + Sync + Send + 'a,
    {
        Ok(other.update(msg).await?.map(mapper))
    }

    async fn forward_init<'a, F, C>(
        other: &'a mut C,
        props: C::Props,
        mapper: F,
    ) -> anyhow::Result<ComponentReturn<Self::Msg>>
    where
        F: Fn(C::Msg) -> Self::Msg + Send + Sync + 'a,
        C: Component + Sync + Send + 'a,
    {
        Ok(other.init(props).await?.map(mapper))
    }

    fn forward_event<F, C>(
        other: &mut C,
        evt: ComponentEvent,
        mapper: F,
    ) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>>
    where
        F: Fn(C::Msg) -> Self::Msg,
        C: Component,
    {
        Ok(other
            .handle_event(evt)?
            .into_iter()
            .map(|c| c.map(&mapper))
            .collect())
    }
}

#[derive(Debug)]
pub struct ComponentMsg<T>(T);

#[derive(Default)]
pub struct ComponentReturn<'a, T> {
    pub(crate) msgs: Vec<ComponentMsg<T>>,
    pub(crate) cmds: Vec<BoxFuture<'a, anyhow::Result<Vec<ComponentMsg<T>>>>>,
    pub(crate) actions: Vec<Action>,
}

impl<'a, T: Debug> Debug for ComponentReturn<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentReturn")
            .field("msgs", &self.msgs)
            .field("cmds", &self.cmds.len())
            .finish()
    }
}

pub enum Action {
    Quit,
    Esc,
    NavTo(Nav),
    ChangeSheet,
    Notification(Notification),
    ClearNotification,
    Spawn(BoxFuture<'static, anyhow::Result<Action>>),
}

impl Action {
    pub fn spawn(fut: impl Future<Output = anyhow::Result<Action>> + Send + 'static) -> Action {
        Action::Spawn(fut.boxed())
    }
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quit => write!(f, "Quit"),
            Self::Esc => write!(f, "Esc"),
            Self::NavTo(arg0) => f.debug_tuple("NavTo").field(arg0).finish(),
            Self::ChangeSheet => write!(f, "ChangeSheet"),
            Self::Notification(arg0) => f.debug_tuple("Notification").field(arg0).finish(),
            Self::Spawn(_arg0) => f.debug_tuple("Spawn").finish(),
            Self::ClearNotification => f.debug_tuple("ClearNotification").finish(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    msg: String,
    kind: NotificationKind,
    timeout: u64,
}

#[derive(Debug, Clone)]
pub enum NotificationMsg {
    Show(Notification),
    Clear,
}

impl Notification {
    pub fn error(msg: String) -> Notification {
        Notification {
            msg,
            kind: NotificationKind::Error,
            timeout: 5,
        }
    }

    pub fn info(msg: String) -> Notification {
        Notification {
            msg,
            kind: NotificationKind::Info,
            timeout: 5,
        }
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn kind(&self) -> &NotificationKind {
        &self.kind
    }

    pub fn timeout(&self) -> u64 {
        self.timeout
    }
}

#[derive(Debug, Clone)]
pub enum NotificationKind {
    Error,
    Info,
}

impl<T> ComponentMsg<T> {
    pub fn to_owned(self) -> T {
        self.0
    }

    pub fn map<M, F>(self, mapper: F) -> ComponentMsg<M>
    where
        F: FnOnce(T) -> M,
    {
        ComponentMsg(mapper(self.0))
    }
}

impl<'a, T: 'a> ComponentReturn<'a, T> {
    pub fn cmd(cmd: BoxFuture<'a, anyhow::Result<Vec<ComponentMsg<T>>>>) -> ComponentReturn<'a, T> {
        ComponentReturn {
            msgs: vec![],
            cmds: vec![cmd],
            actions: vec![],
        }
    }

    pub fn empty() -> ComponentReturn<'a, T> {
        ComponentReturn {
            msgs: vec![],
            cmds: vec![],
            actions: vec![],
        }
    }

    pub fn action(action: Action) -> ComponentReturn<'a, T> {
        ComponentReturn {
            msgs: vec![],
            cmds: vec![],
            actions: vec![action],
        }
    }

    pub fn map<M, F>(self, mapper: F) -> ComponentReturn<'a, M>
    where
        F: Fn(T) -> M + Sync + Send + 'a,
    {
        let msgs = self.msgs.into_iter().map(|msg| msg.map(&mapper)).collect();

        let shared = Arc::new(mapper);
        let cmds = self
            .cmds
            .into_iter()
            .map(|fut| {
                let inner_mapper = shared.clone();
                async move {
                    let msgs = fut.await?;

                    Ok(msgs
                        .into_iter()
                        .map(|msg| msg.map(inner_mapper.as_ref()))
                        .collect())
                }
                .boxed()
            })
            .collect::<Vec<_>>();

        ComponentReturn {
            msgs,
            cmds,
            actions: self.actions,
        }
    }
}

impl<T> From<T> for ComponentMsg<T> {
    fn from(value: T) -> Self {
        ComponentMsg(value)
    }
}

impl<T> From<ComponentMsg<T>> for ComponentReturn<'_, T> {
    fn from(value: ComponentMsg<T>) -> Self {
        ComponentReturn {
            msgs: vec![value],
            cmds: vec![],
            actions: vec![],
        }
    }
}

pub trait ActionHandler {
    type Msg;
    fn handle_action(&mut self, action: Action) -> anyhow::Result<Vec<ComponentMsg<Self::Msg>>>;
}

#[derive(Clone)]
pub enum ComponentEvent {
    Event(Event),
}
