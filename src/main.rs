use rrule::{RRuleResult, RRuleSet};

fn rrule_from_string(rrule: &str) -> RRuleResult {
    let rule: RRuleSet = rrule.parse().unwrap();
    rule.all(100)
}

fn main() {
    let result = rrule_from_string("DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3");
    println!("Result is {:?}", &result)
}
