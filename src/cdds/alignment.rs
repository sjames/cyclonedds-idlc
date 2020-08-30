// Copyright (C) 2020  Sojan James
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>

use std::fmt;

#[derive(PartialEq, Clone)]
pub enum AlignmentType {
    One,
    Bool,
    OneOrBool,
    Two,
    TwoOrBool,
    Four,
    Ptr,
    Eight,
}
#[derive(Clone)]
pub struct Alignment {
    alignment: AlignmentType,
    value: u32,
    ordering: u32,
    rendering: &'static str,
}

impl Alignment {
    pub fn new(alignment: AlignmentType) -> Self {
        match alignment {
            AlignmentType::One => Alignment {
                alignment,
                value: 1,
                ordering: 0,
                rendering: "1u",
            },
            AlignmentType::Bool => Alignment {
                alignment,
                value: 0,
                ordering: 0,
                rendering: "sizeof(bool)",
            },
            AlignmentType::OneOrBool => Alignment {
                alignment,
                value: 0,
                ordering: 1,
                rendering: "(sizeof(bool)>1u)?sizeof(bool):1u)",
            },
            AlignmentType::Two => Alignment {
                alignment,
                value: 2,
                ordering: 2,
                rendering: "2u",
            },
            AlignmentType::TwoOrBool => Alignment {
                alignment,
                value: 0,
                ordering: 3,
                rendering: "(sizeof(bool)>2u)?sizeof(bool):2u",
            },
            AlignmentType::Four => Alignment {
                alignment,
                value: 4,
                ordering: 4,
                rendering: "4u",
            },
            AlignmentType::Ptr => Alignment {
                alignment,
                value: 0,
                ordering: 6,
                rendering: "sizeof (char *)",
            },
            AlignmentType::Eight => Alignment {
                alignment,
                value: 8,
                ordering: 8,
                rendering: "8u",
            },
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value as i32
    }

    pub fn get_ordering(&self) -> i32 {
        self.ordering as i32
    }

    pub fn to_str(&self) -> &'static str {
        self.rendering
    }

    pub fn maximum(self, rhs: Self) -> Self {
        if rhs.alignment == AlignmentType::Bool {
            if self.alignment == AlignmentType::One {
                return Alignment::new(AlignmentType::OneOrBool);
            } else if self.alignment == AlignmentType::Two {
                return Alignment::new(AlignmentType::TwoOrBool);
            } else {
                return self;
            }
        } else {
            if rhs.ordering > self.ordering {
                rhs
            } else {
                self
            }
        }
    }
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rendering)
    }
}

/*
public Alignment maximum (Alignment rhs)
  {
    if (rhs.equals (BOOL))
    {
      if (this.equals (ONE))
      {
        return ONE_OR_BOOL;
      }
      else if (this.equals (TWO))
      {
        return TWO_OR_BOOL;
      }
      else
      {
        return this;
      }
    }
    if (rhs.ordering > ordering)
    {
      return rhs;
    }
    else
    {
      return this;
    }
  }
*/
