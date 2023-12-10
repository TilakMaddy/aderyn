use crate::context::loader::ContextLoader;
use std::{
    io::{Result, Write},
    path::{Path, PathBuf},
};

use super::{
    printer::ReportPrinter,
    reporter::{Issue, Report},
};

pub struct MarkdownReportPrinter;

impl ReportPrinter<()> for MarkdownReportPrinter {
    fn print_report<W: Write>(
        &self,
        mut writer: W,
        report: &Report,
        loader: &ContextLoader,
        root_path: PathBuf,
    ) -> Result<()> {
        self.print_title_and_disclaimer(&mut writer)?;
        self.print_table_of_contents(&mut writer, report)?;
        self.print_contract_summary(&mut writer, report, loader)?;
        let mut counter = 0;
        if !report.criticals.is_empty() {
            writeln!(writer, "# Critical Issues\n")?;
            for issue in &report.criticals {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "C", counter, &root_path)?;
            }
        }
        if !report.highs.is_empty() {
            writeln!(writer, "# High Issues\n")?;
            counter = 0;
            for issue in &report.highs {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "H", counter, &root_path)?;
            }
        }
        if !report.mediums.is_empty() {
            writeln!(writer, "# Medium Issues\n")?;
            counter = 0;
            for issue in &report.mediums {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "M", counter, &root_path)?;
            }
        }
        if !report.lows.is_empty() {
            writeln!(writer, "# Low Issues\n")?;
            counter = 0;
            for issue in &report.lows {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "L", counter, &root_path)?;
            }
        }
        if !report.ncs.is_empty() {
            writeln!(writer, "# NC Issues\n")?;
            counter = 0;
            for issue in &report.ncs {
                counter += 1;
                self.print_issue(&mut writer, issue, loader, "NC", counter, &root_path)?;
            }
        }
        Ok(())
    }
}

impl MarkdownReportPrinter {
    fn print_title_and_disclaimer<W: Write>(&self, mut writer: W) -> Result<()> {
        writeln!(writer, "# Aderyn Analysis Report\n")?;
        writeln!(
            writer,
            "This report was generated by [Aderyn](https://github.com/Cyfrin/aderyn), a static analysis tool \
            built by [Cyfrin](https://cyfrin.io), a blockchain security company. This report is not a substitute for manual audit or security review. \
            It should not be relied upon for any purpose other than to assist in the identification of potential security vulnerabilities."
        )?;
        Ok(())
    }
    fn print_contract_summary<W: Write>(
        &self,
        mut writer: W,
        report: &Report,
        loader: &ContextLoader,
    ) -> Result<()> {
        writeln!(writer, "# Summary\n")?;

        // Files Summary
        writeln!(writer, "## Files Summary\n")?;
        let total_source_units = loader.source_units.len();
        let total_sloc = loader.sloc_stats.code;

        // Start the markdown table
        writeln!(writer, "| Key | Value |")?;
        writeln!(writer, "| --- | --- |")?;
        writeln!(writer, "| .sol Files | {} |", total_source_units)?;
        writeln!(writer, "| Total nSLOC | {} |", total_sloc)?;

        writeln!(writer, "\n")?; // Add an extra newline for spacing

        // Files Details
        writeln!(writer, "## Files Details\n")?;

        // Start the markdown table with the header
        writeln!(writer, "| Filepath | nSLOC |")?;
        writeln!(writer, "| --- | --- |")?;

        let sloc_stats = &loader.sloc_stats;

        let mut source_units = loader.source_units.clone();
        source_units.sort_by_key(|su: &crate::ast::SourceUnit| {
            su.absolute_path.as_deref().unwrap_or("").to_string()
        });

        // Iterate over source units and add each as a row in the markdown table
        for source_unit in source_units {
            let filepath = source_unit.absolute_path.as_ref().unwrap();
            let report: &tokei::Report = sloc_stats
                .reports
                .iter()
                .find(|r| r.name.to_str().map_or(false, |s| s.contains(filepath)))
                .unwrap();
            writeln!(writer, "| {} | {} |", filepath, report.stats.code)?;
        }
        writeln!(writer, "| **Total** | **{}** |", sloc_stats.code)?;
        writeln!(writer, "\n")?; // Add an extra newline for spacing

        // Analysis Sumarry
        writeln!(writer, "## Issue Summary\n")?;
        // Start the markdown table
        writeln!(writer, "| Category | No. of Issues |")?;
        writeln!(writer, "| --- | --- |")?;
        writeln!(writer, "| Critical | {} |", report.criticals.len())?;
        writeln!(writer, "| High | {} |", report.highs.len())?;
        writeln!(writer, "| Medium | {} |", report.mediums.len())?;
        writeln!(writer, "| Low | {} |", report.lows.len())?;
        writeln!(writer, "| NC | {} |", report.ncs.len())?;
        writeln!(writer, "\n")?; // Add an extra newline for spacing

        Ok(())
    }

    fn print_table_of_contents<W: Write>(&self, mut writer: W, report: &Report) -> Result<()> {
        writeln!(writer, "# Table of Contents\n")?;
        writeln!(writer, "- [Summary](#summary)")?;
        writeln!(writer, "  - [Files Summary](#files-summary)")?;
        writeln!(writer, "  - [Files Details](#files-details)")?;
        writeln!(writer, "  - [Issue Summary](#issue-summary)")?;
        if !report.criticals.is_empty() {
            writeln!(writer, "- [Critical Issues](#critical-issues)")?;
            for (index, issue) in report.criticals.iter().enumerate() {
                let issue_title_slug = issue
                    .title
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");
                writeln!(
                    writer,
                    "  - [C-{}: {}](#C-{}-{})",
                    index + 1,
                    issue.title,
                    index + 1,
                    issue_title_slug
                )?;
            }
        }
        if !report.highs.is_empty() {
            writeln!(writer, "- [High Issues](#high-issues)")?;
            for (index, issue) in report.highs.iter().enumerate() {
                let issue_title_slug = issue
                    .title
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");
                writeln!(
                    writer,
                    "  - [H-{}: {}](#H-{}-{})",
                    index + 1,
                    issue.title,
                    index + 1,
                    issue_title_slug
                )?;
            }
        }
        if !report.mediums.is_empty() {
            writeln!(writer, "- [Medium Issues](#medium-issues)")?;
            for (index, issue) in report.mediums.iter().enumerate() {
                let issue_title_slug = issue
                    .title
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");
                writeln!(
                    writer,
                    "  - [M-{}: {}](#M-{}-{})",
                    index + 1,
                    issue.title,
                    index + 1,
                    issue_title_slug
                )?;
            }
        }
        if !report.lows.is_empty() {
            writeln!(writer, "- [Low Issues](#low-issues)")?;
            for (index, issue) in report.lows.iter().enumerate() {
                let issue_title_slug = issue
                    .title
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");
                writeln!(
                    writer,
                    "  - [L-{}: {}](#L-{}-{})",
                    index + 1,
                    issue.title,
                    index + 1,
                    issue_title_slug
                )?;
            }
        }
        if !report.ncs.is_empty() {
            writeln!(writer, "- [NC Issues](#nc-issues)")?;
            for (index, issue) in report.ncs.iter().enumerate() {
                let issue_title_slug = issue
                    .title
                    .to_lowercase()
                    .replace(' ', "-")
                    .replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");
                writeln!(
                    writer,
                    "  - [NC-{}: {}](#NC-{}-{})",
                    index + 1,
                    issue.title,
                    index + 1,
                    issue_title_slug
                )?;
            }
        }
        writeln!(writer, "\n")?; // Add an extra newline for spacing
        Ok(())
    }

    fn print_issue<W: Write>(
        &self,
        mut writer: W,
        issue: &Issue,
        _loader: &ContextLoader,
        severity: &str,
        number: i32,
        root_path: &Path,
    ) -> Result<()> {
        let is_file = root_path.is_file();

        writeln!(
            writer,
            "## {}-{}: {}\n\n{}\n", // <a name> is the anchor for the issue title
            severity, number, issue.title, issue.description
        )?;
        for (contract_path, line_number) in issue.instances.keys() {
            let path = {
                if is_file {
                    String::from(root_path.to_str().unwrap())
                } else {
                    String::from(root_path.join(contract_path).as_path().to_str().unwrap())
                }
            };

            let line = std::fs::read_to_string(&path).unwrap();

            let line_preview = line
                .split("\n")
                .into_iter()
                .skip(line_number - 1)
                .take(1)
                .next()
                .unwrap();

            writeln!(
                writer,
                "- Found in {} Line: {}\n\n\t>{}\n\nfile://{}\n\n\n",
                contract_path, line_number, line_preview, &path,
            )?;
        }
        writeln!(writer, "\n")?; // Add an extra newline for spacing
        Ok(())
    }
}
