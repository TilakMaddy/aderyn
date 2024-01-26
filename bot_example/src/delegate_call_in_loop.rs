use std::collections::BTreeMap;
use std::error::Error;

use aderyn_driver::core_ast::MemberAccess;

use aderyn_driver::context::{browser::ExtractMemberAccesses, loader::ContextLoader};
use aderyn_driver::detection_modules::capture;
use aderyn_driver::detector::{Detector, IssueSeverity};

#[derive(Default)]
pub struct DelegateCallInLoopDetector {
    // Keys are source file name and line number
    found_instances: BTreeMap<(String, usize), String>,
}

impl Detector for DelegateCallInLoopDetector {
    fn detect(&mut self, loader: &ContextLoader) -> Result<bool, Box<dyn Error>> {
        let mut member_accesses: Vec<MemberAccess> = vec![];

        // Get all delegatecall member accesses inside for statements
        member_accesses.extend(loader.for_statements.keys().flat_map(|statement| {
            ExtractMemberAccesses::from(statement)
                .extracted
                .into_iter()
                .filter(|ma| ma.member_name == "delegatecall")
        }));

        // Get all delegatecall member accsesses inside while statements
        member_accesses.extend(loader.while_statements.keys().flat_map(|statement| {
            ExtractMemberAccesses::from(statement)
                .extracted
                .into_iter()
                .filter(|ma| ma.member_name == "delegatecall")
        }));

        // For each member access found, add them to found_instances
        for member_access in member_accesses {
            capture!(self, loader, member_access);
        }

        Ok(!self.found_instances.is_empty())
    }

    fn severity(&self) -> IssueSeverity {
        IssueSeverity::High
    }

    fn title(&self) -> String {
        String::from("Using `delegatecall` in loop")
    }

    fn description(&self) -> String {
        String::from("When calling `delegatecall` the same `msg.value` amount will be accredited multiple times.")
    }

    fn instances(&self) -> BTreeMap<(String, usize), String> {
        self.found_instances.clone()
    }
}

#[cfg(test)]
mod delegate_call_in_loop_detector_tests {

    use super::DelegateCallInLoopDetector;
    use aderyn_driver::detector::detector_test_helpers::load_contract;
    use aderyn_driver::detector::{Detector, IssueSeverity};

    #[test]
    fn test_delegate_call_in_loop_detector() {
        let context_loader = load_contract(
            "../../aderyn/tests/contract-playground/out/ExtendedInheritance.sol/ExtendedInheritance.json",
        );

        let mut detector = DelegateCallInLoopDetector::default();
        let found = detector.detect(&context_loader).unwrap();
        // assert that the detector found a delegate call in a loop
        assert!(found);
        // assert that the detector found the correct number of instances (1)
        assert_eq!(detector.instances().len(), 1);
        // assert the severity is high
        assert_eq!(detector.severity(), IssueSeverity::High);
        // assert the title is correct
        assert_eq!(
            detector.title(),
            String::from("Using `delegatecall` in loop")
        );
        // assert the description is correct
        assert_eq!(
            detector.description(),
            String::from(
                "When calling `delegatecall` the same `msg.value` amount will be accredited multiple times."
            )
        );
    }
}
