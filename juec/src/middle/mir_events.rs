// src/mir_events.rs
use mir::{NodeId, NodeKind};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::middle::mir;

#[derive(Debug, Clone)]
pub enum EditOp {
    Insert,
    Replace,
    Delete,
}

#[derive(Debug, Clone)]
pub struct EditEvent {
    pub op: EditOp,
    pub node_id: NodeId,
    pub parent: Option<NodeId>,
    pub position: Option<usize>,
    pub old: Option<NodeKind>,
    pub new: Option<NodeKind>,
    pub actor: Option<String>, // agent id
    pub ts: u64,
}

impl EditEvent {
    fn now_ts() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    pub fn insert(
        node_id: NodeId,
        parent: Option<NodeId>,
        position: Option<usize>,
        actor: Option<String>,
    ) -> Self {
        EditEvent {
            op: EditOp::Insert,
            node_id,
            parent,
            position,
            old: None,
            new: None,
            actor,
            ts: Self::now_ts(),
        }
    }

    pub fn replace(node_id: NodeId, old: NodeKind, new: NodeKind, actor: Option<String>) -> Self {
        EditEvent {
            op: EditOp::Replace,
            node_id,
            parent: None,
            position: None,
            old: Some(old),
            new: Some(new),
            actor,
            ts: Self::now_ts(),
        }
    }

    pub fn delete(node_id: NodeId, old: NodeKind, actor: Option<String>) -> Self {
        EditEvent {
            op: EditOp::Delete,
            node_id,
            parent: None,
            position: None,
            old: Some(old),
            new: None,
            actor,
            ts: Self::now_ts(),
        }
    }
}
