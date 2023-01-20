use std::{marker::PhantomData, ops::Range};

use crate::utils::{GetBits, TestBits};

#[derive(Clone, Copy)]
struct Pattern {
    pattern: u32,
    mask: u32,
}

pub struct PatternMatcher<T> {
    args: Vec<(String, Range<u8>)>,
    patterns: Vec<(T, Pattern)>,
}

impl<T> PatternMatcher<T>
where
    T: Copy,
{
    pub fn builder() -> PatternMatcherBuilder<T, NeedArgs> {
        let result = PatternMatcher {
            args: Vec::new(),
            patterns: Vec::new(),
        };
        let pattern = Pattern {
            pattern: 0,
            mask: 0,
        };

        PatternMatcherBuilder {
            result: result,
            current_instr: None,
            current_pattern: pattern,
            current_index: 0,
            _build_state: PhantomData,
        }
    }

    pub fn get_arg(&self, val: u32, idx: usize) -> u32 {
        val.get_bits(&self.args[idx].1)
    }

    pub fn match_pattern(&self, target: u32) -> Option<T> {
        for pat in &self.patterns {
            if target.test_bits(pat.1.pattern, pat.1.mask) {
                return Some(pat.0);
            } else {
                continue;
            }
        }

        None
    }
}

pub struct NeedArgs;
pub struct NeedPattern;
pub struct With;

pub struct PatternMatcherBuilder<T, BuildState>
where
    T: Copy,
{
    result: PatternMatcher<T>,

    current_instr: Option<T>,
    current_pattern: Pattern,
    current_index: usize,

    _build_state: PhantomData<BuildState>,
}

impl<T> PatternMatcherBuilder<T, NeedArgs>
where
    T: Copy,
{
    pub fn args(
        mut self,
        arg_name: &str,
        arg_range: Range<u8>,
    ) -> PatternMatcherBuilder<T, NeedArgs> {
        self.result.args.push((arg_name.to_string(), arg_range));

        self
    }

    pub fn inst(mut self, instr: T) -> PatternMatcherBuilder<T, NeedPattern> {
        self.current_instr = Some(instr);

        self.current_pattern = Pattern {
            pattern: 0,
            mask: 0,
        };

        PatternMatcherBuilder {
            result: self.result,
            current_instr: self.current_instr,
            current_pattern: self.current_pattern,
            current_index: 0,
            _build_state: PhantomData,
        }
    }
}

impl<T> PatternMatcherBuilder<T, NeedPattern>
where
    T: Copy,
{
    fn parse_pattern(pattern: &str, range: &Range<u8>) -> (u32, u32) {
        let mut pattern_result = 0b0;
        let mut mask_result = 0b0;

        let mut count = 0;

        for char in pattern.chars() {
            let (pat, mask) = match char {
                'x' => (0, 0),
                '0' => (0, 1),
                '1' => (1, 1),
                '_' => {
                    continue;
                }
                _ => unreachable!(),
            };
            pattern_result <<= 1;
            pattern_result |= pat;

            mask_result <<= 1;
            mask_result |= mask;

            count += 1;
        }

        assert_eq!(
            count,
            range.len(),
            "Pattern string doesn't match with pattern range in pattern: {:#?}",
            pattern
        );

        pattern_result <<= range.start;
        mask_result <<= range.start;

        (pattern_result, mask_result)
    }

    fn push_current(&mut self) {
        self.result
            .patterns
            .push((self.current_instr.unwrap(), self.current_pattern));

        self.current_pattern = Pattern {
            pattern: 0,
            mask: 0,
        };
    }

    pub fn with(mut self, pattern: &str) -> PatternMatcherBuilder<T, NeedPattern> {
        let range = &self.result.args[self.current_index].1;
        self.current_index += 1;

        let (pat, mask) = Self::parse_pattern(pattern, range);

        self.current_pattern.pattern |= pat;
        self.current_pattern.mask |= mask;

        self
    }

    pub fn inst(mut self, instr: T) -> PatternMatcherBuilder<T, NeedPattern> {
        self.push_current();

        self.current_instr = Some(instr);

        PatternMatcherBuilder {
            result: self.result,
            current_instr: self.current_instr,
            current_pattern: self.current_pattern,
            current_index: 0,
            _build_state: PhantomData,
        }
    }

    pub fn build(mut self) -> PatternMatcher<T> {
        self.push_current();

        self.result
    }
}

#[cfg(test)]
mod tests {
    use std::panic::PanicInfo;

    use super::*;

    #[derive(Clone, Copy)]
    enum MockPattern {
        Pattern1,
        Pattern2,
        Pattern3,
    }

    #[test]
    fn test_one_pattern_match() {
        let pattern_matcher = PatternMatcher::<MockPattern>::builder()
            .args("op0", 0..4)
            .inst(MockPattern::Pattern1)
            .with("x000")
            .inst(MockPattern::Pattern2)
            .with("11xx")
            .inst(MockPattern::Pattern3)
            .with("0011")
            .build();

        match pattern_matcher.match_pattern(0b0000) {
            Some(MockPattern::Pattern1) => assert!(true),
            Some(MockPattern::Pattern2) => unreachable!(),
            Some(MockPattern::Pattern3) => unreachable!(),
            None => unreachable!(),
        }

        match pattern_matcher.match_pattern(0b1111) {
            Some(MockPattern::Pattern1) => unreachable!(),
            Some(MockPattern::Pattern2) => assert!(true),
            Some(MockPattern::Pattern3) => unreachable!(),
            None => unreachable!(),
        }

        match pattern_matcher.match_pattern(0b0011) {
            Some(MockPattern::Pattern1) => unreachable!(),
            Some(MockPattern::Pattern2) => unreachable!(),
            Some(MockPattern::Pattern3) => assert!(true),
            None => unreachable!(),
        }

        match pattern_matcher.match_pattern(0b1010) {
            Some(MockPattern::Pattern1) => unreachable!(),
            Some(MockPattern::Pattern2) => unreachable!(),
            Some(MockPattern::Pattern3) => unreachable!(),
            None => assert!(true),
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_pattern_length() {
        let f = |_: &PanicInfo| {};
        std::panic::set_hook(Box::new(f));

        let _ = PatternMatcher::<MockPattern>::builder()
            .args("op0", 0..4)
            .inst(MockPattern::Pattern1)
            .with("x0x0x0x0")
            .build();
    }

    #[test]
    fn test_multiple_pattern_match() {
        let pattern_matcher = PatternMatcher::<MockPattern>::builder()
            .args("op0", 0..4)
            .args("op1", 31..32)
            .args("op2", 10..16)
            .inst(MockPattern::Pattern1)
            .with("x000")
            .with("1")
            .with("xxx_xx1")
            .inst(MockPattern::Pattern2)
            .with("11xx")
            .with("0")
            .with("010_xx1")
            .inst(MockPattern::Pattern3)
            .with("0011")
            .with("0")
            .with("000_xx1")
            .build();

        match pattern_matcher.match_pattern(0b1_000_0000_0000_0000_000001_00_0000_0000) {
            Some(MockPattern::Pattern1) => assert!(true),
            Some(MockPattern::Pattern2) => unreachable!(),
            Some(MockPattern::Pattern3) => unreachable!(),
            None => unreachable!(),
        }

        match pattern_matcher.match_pattern(0b0_000_0000_0000_0000_010111_00_0000_1110) {
            Some(MockPattern::Pattern1) => unreachable!(),
            Some(MockPattern::Pattern2) => assert!(true),
            Some(MockPattern::Pattern3) => unreachable!(),
            None => unreachable!(),
        }

        match pattern_matcher.match_pattern(0b0_000_0000_0000_0000_000111_00_0000_0011) {
            Some(MockPattern::Pattern1) => unreachable!(),
            Some(MockPattern::Pattern2) => unreachable!(),
            Some(MockPattern::Pattern3) => assert!(true),
            None => unreachable!(),
        }

        match pattern_matcher.match_pattern(0b1_000_0000_0000_0000_010001_00_0000_1111) {
            Some(MockPattern::Pattern1) => unreachable!(),
            Some(MockPattern::Pattern2) => unreachable!(),
            Some(MockPattern::Pattern3) => unreachable!(),
            None => assert!(true),
        }
    }
}
