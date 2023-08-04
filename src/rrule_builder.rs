use rrule::{RRuleResult, RRuleSet};

pub fn rrule_from_string(rrule: &String) -> RRuleResult {
    let rrule_set: RRuleSet = rrule.parse().unwrap();
    rrule_set.all(100)
}
