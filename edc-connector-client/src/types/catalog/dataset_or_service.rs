use super::{Dataset, Service};

#[derive(Clone, PartialEq, Debug)]
pub enum DatasetOrService {
    Dataset(Dataset),
    Service(Service),
}
