use std::{collections::BTreeMap, error::Error};

use crate::{
    ast::NodeID,
    capture,
    context::{browser::GetParent, workspace_context::WorkspaceContext},
    detect::detector::{IssueDetector, IssueSeverity},
};

#[derive(Default)]
pub struct WeirdErc20NotHandledDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), NodeID>,
}

impl IssueDetector for WeirdErc20NotHandledDetector {
    fn detect(&mut self, context: &WorkspaceContext) -> Result<bool, Box<dyn Error>> {
        for identifier in context.identifiers.keys() {
            let source_unit = GetParent::source_unit_of(identifier, context).unwrap();

            let import_directives = source_unit.import_directives();
            if import_directives.iter().any(|directive| {
                directive
                    .absolute_path
                    .as_ref()
                    .map_or(false, |path| path.contains("openzeppelin"))
            }) && identifier.name == "_mint"
            {
                capture!(self, context, identifier);
            }
        }
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
