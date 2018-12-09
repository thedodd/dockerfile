use std::{
    borrow::Cow,
    fmt,
};

use crate::{
    instructions::{
        Arg,
        Directive,
        From,
        Instruction,
    },
};

/// A Dockerfile represented in code.
///
/// A Dockerfile, conceptually, is a series of instructions. In code, that is exactly how they are
/// represented here. A wrapper around a `Vec<Instruction>` with a few convenience methods and
/// such.
pub struct Dockerfile(Vec<Instruction>);

impl fmt::Display for Dockerfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().fold(String::new(), |acc, elem| acc + &elem.to_string()))
    }
}

impl Dockerfile {
    /// Start building a new Dockerfile from the specified base image.
    ///
    /// Only requirement is the initial `FROM` instruction. Call `.finish()` when complete.
    pub fn base<T: Into<Cow<'static, str>>>(from: T) -> DockerfileBuilder {
        DockerfileBuilder{
            initial_directives: None,
            initial_args: None,
            from: From::new(from),
            instructions: None,
        }
    }
}

/// A Dockerfile builder.
pub struct DockerfileBuilder {
    /// Any parser directives.
    initial_directives: Option<Vec<Directive>>,

    /// Any initial `ARG`s which are to appear before the initial `FROM` instruction.
    initial_args: Option<Vec<Arg>>,

    /// The new Dockerfile's initial `FROM` instruction.
    from: From,

    /// Any additional instructions part of the Dockerfile.
    instructions: Option<Vec<Instruction>>,
}

impl DockerfileBuilder {
    /// Push a new initial directive to this Dockerfile instance.
    pub fn push_initial_directive(mut self, directive: Directive) -> Self {
        if let Some(ref mut directives) = self.initial_directives {
            directives.push(directive);
        } else {
            self.initial_directives = Some(vec![directive]);
        }
        self
    }

    /// Push a new initial arg to this Dockerfile instance.
    pub fn push_initial_arg(mut self, arg: Arg) -> Self {
        if let Some(ref mut args) = self.initial_args {
            args.push(arg);
        } else {
            self.initial_args = Some(vec![arg]);
        }
        self
    }

    /// Push a new instruction into this Dockerfile instance.
    pub fn push<I: Into<Instruction>>(mut self, instruction: I) -> Self {
        if let Some(ref mut instructions) = self.instructions {
            instructions.push(instruction.into());
        } else {
            self.instructions = Some(vec![instruction.into()]);
        }
        self
    }

    /// Append a vector of instructions to this Dockerfile instance.
    pub fn append<I: Into<Instruction>>(mut self, new: Vec<I>) -> Self {
        let mut new = new.into_iter().map(Into::into).collect();
        if let Some(ref mut instructions) = self.instructions {
            instructions.append(&mut new);
        } else {
            self.instructions = Some(new);
        }
        self
    }

    /// Generate the output Dockerfile as a string.
    pub fn finish(self) -> Dockerfile {
        // Add directives to the Dockerfile.
        let mut all_instructions: Vec<Instruction> = vec![];
        if let Some(directives) = self.initial_directives {
            all_instructions.extend(directives.into_iter().map(|inst| Instruction::Directive(inst)));
        }

        // Add initial args to the Dockerfile.
        if let Some(args) = self.initial_args {
            all_instructions.extend(args.into_iter().map(|inst| Instruction::Arg(inst)));
        }

        // Add from instruction to Dockerfile.
        all_instructions.push(Instruction::From(self.from));

        // Append any other instructions in serial order.
        if let Some(instructions) = self.instructions {
            all_instructions.extend(instructions.into_iter());
        }

        Dockerfile(all_instructions)
    }
}
