use std::{
    borrow::Cow,
    convert,
    fmt,
};

/// The `ARG` instruction defines a variable that users can pass at build-time to the builder with
/// the `docker build` command using the `--build-arg <varname>=<value>` flag.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#arg
pub struct Arg(Cow<'static, str>);

impl Arg {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Arg(val.into())
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ARG {}\n", &self.0)
    }
}

/// The main purpose of a `CMD` is to provide defaults for an executing container.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#cmd
pub struct Cmd(Cow<'static, str>);

impl Cmd {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Cmd(val.into())
    }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CMD {}\n", &self.0)
    }
}

/// The `COPY` instruction copies new files or directories from `<src>` and adds them to the
/// filesystem of the container at the path `<dest>`.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#copy
pub struct Copy(Cow<'static, str>);

impl Copy {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Copy(val.into())
    }
}

impl fmt::Display for Copy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "COPY {}\n", &self.0)
    }
}

/// Parser directives are optional, and affect the way in which subsequent lines in a `Dockerfile`
/// are handled.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#parser-directives
pub struct Directive(Cow<'static, str>);

impl Directive {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Directive(val.into())
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "# {}\n", &self.0)
    }
}

/// An `ENTRYPOINT` allows you to configure a container that will run as an executable.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#entrypoint
pub struct Entrypoint(Cow<'static, str>);

impl Entrypoint {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Entrypoint(val.into())
    }
}

impl fmt::Display for Entrypoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ENTRYPOINT {}\n", &self.0)
    }
}

/// The `FROM` instruction initializes a new build stage and sets the base image for subsequent
/// instructions.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#from
pub struct From(Cow<'static, str>);

impl From {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        From(val.into())
    }
}

impl fmt::Display for From {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FROM {}\n", &self.0)
    }
}

/// The `RUN` instruction will execute any commands in a new layer on top of the current image and
/// commit the results.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#run
pub struct Run(Cow<'static, str>);

impl Run {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Run(val.into())
    }
}

impl fmt::Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RUN {}\n", &self.0)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// Instruction Enum //////////////////////////////////////////////////////////////////////////////

pub enum Instruction {
    Arg(Arg),
    Cmd(Cmd),
    Copy(Copy),
    Directive(Directive),
    Entrypoint(Entrypoint),
    From(From),
    Run(Run),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Arg(inst) => write!(f, "{}", inst),
            Instruction::Cmd(inst) => write!(f, "{}", inst),
            Instruction::Copy(inst) => write!(f, "{}", inst),
            Instruction::Directive(inst) => write!(f, "{}", inst),
            Instruction::Entrypoint(inst) => write!(f, "{}", inst),
            Instruction::From(inst) => write!(f, "{}", inst),
            Instruction::Run(inst) => write!(f, "{}", inst),
        }
    }
}

impl convert::From<Arg> for Instruction {
    fn from(inst: Arg) -> Self {
        Instruction::Arg(inst)
    }
}

impl convert::From<Cmd> for Instruction {
    fn from(inst: Cmd) -> Self {
        Instruction::Cmd(inst)
    }
}

impl convert::From<Copy> for Instruction {
    fn from(inst: Copy) -> Self {
        Instruction::Copy(inst)
    }
}

impl convert::From<Directive> for Instruction {
    fn from(inst: Directive) -> Self {
        Instruction::Directive(inst)
    }
}

impl convert::From<Entrypoint> for Instruction {
    fn from(inst: Entrypoint) -> Self {
        Instruction::Entrypoint(inst)
    }
}

impl convert::From<From> for Instruction {
    fn from(inst: From) -> Self {
        Instruction::From(inst)
    }
}

impl convert::From<Run> for Instruction {
    fn from(inst: Run) -> Self {
        Instruction::Run(inst)
    }
}
