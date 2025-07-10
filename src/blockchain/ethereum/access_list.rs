use crate::blockchain::ethereum::AccessListItem;
use wtx::collection::Vector;

/// Access list
pub type AccessList = Vector<AccessListItem>;
