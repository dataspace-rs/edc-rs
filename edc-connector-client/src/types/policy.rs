mod odrl;

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::ConversionError;

use super::properties::{FromValue, Properties, PropertyValue, ToValue};

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct PolicyDefinition {
    #[builder(field)]
    #[serde(default)]
    private_properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    policy: Policy,
}

impl<S: policy_definition_builder::State> PolicyDefinitionBuilder<S> {
    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }
}

impl PolicyDefinition {
    pub fn policy(&self) -> &Policy {
        &self.policy
    }

    pub fn private_property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.private_properties.get(property)
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NewPolicyDefinition {
    #[builder(field)]
    #[serde(default)]
    private_properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: Option<String>,
    policy: Policy,
}

impl<S: new_policy_definition_builder::State> NewPolicyDefinitionBuilder<S> {
    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }
}

impl Default for PolicyDefinition {
    fn default() -> Self {
        Self {
            id: String::default(),
            policy: Policy::builder().build(),
            private_properties: Properties::default(),
        }
    }
}

impl Default for NewPolicyDefinition {
    fn default() -> Self {
        Self {
            id: Option::default(),
            policy: Policy::builder().build(),
            private_properties: Properties::default(),
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Builder)]
pub struct Policy {
    #[builder(field)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "permission", alias = "odrl:permission", default)]
    permissions: Vec<Permission>,
    #[builder(field)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "obligation", alias = "odrl:obligation", default)]
    obligations: Vec<Obligation>,
    #[builder(field)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "prohibition", alias = "odrl:prohibition", default)]
    prohibitions: Vec<Prohibition>,
    #[builder(into)]
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[builder(default)]
    #[serde(rename = "@type")]
    kind: PolicyKind,
    #[builder(into)]
    #[serde(alias = "odrl:assignee")]
    assignee: Option<String>,
    #[builder(into)]
    #[serde(alias = "odrl:assigner")]
    assigner: Option<String>,
    #[builder(into)]
    #[serde(alias = "odrl:target")]
    target: Option<Target>,
}

impl Policy {
    pub fn kind(&self) -> &PolicyKind {
        &self.kind
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn assignee(&self) -> Option<&String> {
        self.assignee.as_ref()
    }

    pub fn assigner(&self) -> Option<&String> {
        self.assigner.as_ref()
    }

    pub fn target(&self) -> Option<&Target> {
        self.target.as_ref()
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn obligations(&self) -> &[Obligation] {
        &self.obligations
    }

    pub fn prohibitions(&self) -> &[Prohibition] {
        &self.prohibitions
    }
}

impl<S: policy_builder::State> PolicyBuilder<S> {
    pub fn permissions(mut self, permissions: Vec<Permission>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn permission(mut self, permission: Permission) -> Self {
        self.permissions.push(permission);
        self
    }

    pub fn prohibitions(mut self, prohibitions: Vec<Prohibition>) -> Self {
        self.prohibitions = prohibitions;
        self
    }

    pub fn prohibition(mut self, prohibition: Prohibition) -> Self {
        self.prohibitions.push(prohibition);
        self
    }

    pub fn obligations(mut self, obligations: Vec<Obligation>) -> Self {
        self.obligations = obligations;
        self
    }

    pub fn obligation(mut self, obligation: Obligation) -> Self {
        self.obligations.push(obligation);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub enum PolicyKind {
    #[default]
    #[serde(alias = "odrl:Set")]
    Set,
    #[serde(alias = "odrl:Offer")]
    Offer,
    #[serde(alias = "odrl:Agreement")]
    Agreement,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Builder)]
pub struct Permission {
    #[builder(field)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "constraint", alias = "odrl:constraint", default)]
    constraints: Vec<Constraint>,
    #[builder(default)]
    #[serde(alias = "odrl:action")]
    action: Action,
}

impl Permission {
    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

impl<S: permission_builder::State> PermissionBuilder<S> {
    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn constraint(mut self, constraint: Constraint) -> Self {
        self.constraints.push(constraint);
        self
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Builder)]
pub struct Obligation {
    #[builder(field)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "constraint", alias = "odrl:constraint", default)]
    constraints: Vec<Constraint>,
    #[serde(alias = "odrl:action")]
    action: Action,
}

impl Obligation {
    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

impl<S: obligation_builder::State> ObligationBuilder<S> {
    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn constraint(mut self, constraint: Constraint) -> Self {
        self.constraints.push(constraint);
        self
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Builder)]
pub struct Prohibition {
    #[builder(field)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "constraint", alias = "odrl:constraint", default)]
    constraints: Vec<Constraint>,
    #[serde(alias = "odrl:action")]
    action: Action,
}

impl Prohibition {
    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

impl<S: prohibition_builder::State> ProhibitionBuilder<S> {
    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn constraint(mut self, constraint: Constraint) -> Self {
        self.constraints.push(constraint);
        self
    }
}

#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Simple(String),
    Id {
        #[serde(rename = "@id")]
        id: String,
    },
}

#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Target {
    Simple(String),
    Id {
        #[serde(rename = "@id")]
        id: String,
    },
}

impl Target {
    pub fn simple(target: &str) -> Target {
        Target::Simple(target.to_string())
    }

    pub fn id(target: &str) -> Target {
        Target::Id {
            id: target.to_string(),
        }
    }

    pub fn get_id(&self) -> &str {
        match self {
            Target::Simple(target) => target,
            Target::Id { id } => id,
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::new("http://www.w3.org/ns/odrl/2/use".to_string())
    }
}

impl Action {
    pub fn id(&self) -> &String {
        match self {
            Action::Simple(id) => id,
            Action::Id { id } => id,
        }
    }
}

impl Action {
    pub fn new(kind: String) -> Self {
        Action::Id { id: kind }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Constraint {
    Atomic(AtomicConstraint),
    MultiplicityConstraint(MultiplicityConstraint),
}

impl Constraint {
    pub fn atomic(atomic: AtomicConstraint) -> Self {
        Constraint::Atomic(atomic)
    }

    pub fn or(constraints: Vec<Constraint>) -> Self {
        Constraint::MultiplicityConstraint(MultiplicityConstraint::Or(constraints))
    }

    pub fn and(constraints: Vec<Constraint>) -> Self {
        Constraint::MultiplicityConstraint(MultiplicityConstraint::And(constraints))
    }

    pub fn xone(constraints: Vec<Constraint>) -> Self {
        Constraint::MultiplicityConstraint(MultiplicityConstraint::Xone(constraints))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum LeftOperand {
    Simple(String),
    Id {
        #[serde(rename = "@id")]
        id: String,
    },
}

impl LeftOperand {
    pub fn simple(op: &str) -> LeftOperand {
        LeftOperand::Simple(op.to_string())
    }

    pub fn id(op: &str) -> LeftOperand {
        LeftOperand::Id { id: op.to_string() }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AtomicConstraint {
    #[serde(rename = "leftOperand", alias = "odrl:leftOperand")]
    left_operand: LeftOperand,
    #[serde(alias = "odrl:operator")]
    operator: Operator,
    #[serde(rename = "rightOperand", alias = "odrl:rightOperand")]
    right_operand: PropertyValue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MultiplicityConstraint {
    Or(Vec<Constraint>),
    And(Vec<Constraint>),
    Xone(Vec<Constraint>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Operator {
    Simple(String),
    Id {
        #[serde(rename = "@id")]
        id: String,
    },
}

impl Operator {
    pub fn simple(op: &str) -> Operator {
        Operator::Simple(op.to_string())
    }

    pub fn id(op: &str) -> Operator {
        Operator::Id { id: op.to_string() }
    }
}

impl AtomicConstraint {
    pub fn new<T: ToValue>(left_operand: &str, operator: &str, right_operand: T) -> Self {
        AtomicConstraint::new_with_operator(
            LeftOperand::Simple(left_operand.to_string()),
            Operator::Simple(operator.to_string()),
            right_operand,
        )
    }

    pub fn new_with_operator<T: ToValue>(
        left_operand: impl Into<LeftOperand>,
        operator: Operator,
        right_operand: T,
    ) -> Self {
        Self {
            left_operand: left_operand.into(),
            operator,
            right_operand: PropertyValue(right_operand.into_value()),
        }
    }
}

impl From<&str> for LeftOperand {
    fn from(value: &str) -> Self {
        LeftOperand::Id {
            id: value.to_string(),
        }
    }
}
