// Copyright 2019 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

#[derive(Debug, PartialEq)]
pub enum AllianceCmd {
    AddValidator(AddValidator),
    RemoveValidator(RemoveValidator),
}

#[derive(Debug, PartialEq)]
pub struct AddValidator {}

impl AddValidator {
    pub fn new(matches: &clap::ArgMatches) -> Self { Self {} }
}

#[derive(Debug, PartialEq)]
pub struct RemoveValidator {}

impl RemoveValidator {
    pub fn new(matches: &clap::ArgMatches) -> Self { Self {} }
}

pub fn execute(cmd: AllianceCmd) -> Result<String, String> {
    match cmd {
        AllianceCmd::AddValidator(cmd) => add_validator(cmd),
        AllianceCmd::RemoveValidator(cmd) => remove_validator(cmd),
    }
}

fn add_validator(cmd: AddValidator) -> Result<String, String> { Ok("".into()) }

fn remove_validator(cmd: RemoveValidator) -> Result<String, String> {
    Ok("".into())
}
