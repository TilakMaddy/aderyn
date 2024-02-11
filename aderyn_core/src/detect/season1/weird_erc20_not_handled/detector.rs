use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    context::workspace_context::WorkspaceContext,
    detect::detector::{IssueDetector, IssueSeverity},
};

#[derive(Default)]
pub struct WeirdErc20NotHandledDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl IssueDetector for WeirdErc20NotHandledDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        // Use the `context` to find nodes, then capture them as shown below
        // capture!(self, context, ast_node);

        Ok(!self.found_instances.is_empty())
    }

    fn title(&self) -> String {
        String::from("Title for WeirdErc20NotHandledDetector")
    }

    fn description(&self) -> String {
        String::from("Description for WeirdErc20NotHandledDetector")
    }

    fn severity(&self) -> IssueSeverity {
        // Choose the appropriate severity
        IssueSeverity::NC
    }

    fn name(&self) -> String {
        "WeirdErc20NotHandledDetector".to_string()
    }

    fn instances(&self) -> BTreeMap<(String, usize), NodeID> {
        self.found_instances.clone()
    }
}
