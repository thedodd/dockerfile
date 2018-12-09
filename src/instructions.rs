use std::{
    borrow::Cow,
    convert,
    fmt,
};

/// The `ADD` instruction copies new files, directories or remote file URLs from `<src>` and adds
/// them to the filesystem of the image at the path `<dest>`.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#add
pub struct Add(Cow<'static, str>);

impl Add {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Add(val.into())
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADD {}\n", &self.0)
    }
}

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

/// The `ENV` instruction sets the environment variable `<key>` to the value `<value>`.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#env
pub struct Env(Cow<'static, str>);

impl Env {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Env(val.into())
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ENV {}\n", &self.0)
    }
}

/// The `EXPOSE` instruction informs Docker that the container listens on the specified network
/// ports at runtime.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#expose
pub struct Expose(Cow<'static, str>);

impl Expose {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Expose(val.into())
    }
}

impl fmt::Display for Expose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EXPOSE {}\n", &self.0)
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

/// The `HEALTHCHECK` instruction tells Docker how to test a container to check that it is still
/// working.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#healthcheck
pub struct Healthcheck(Cow<'static, str>);

impl Healthcheck {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Healthcheck(val.into())
    }
}

impl fmt::Display for Healthcheck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HEALTHCHECK {}\n", &self.0)
    }
}

/// The `LABEL` instruction adds metadata to an image.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#label
pub struct Label(Cow<'static, str>);

impl Label {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Label(val.into())
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LABEL {}\n", &self.0)
    }
}

/// The `ONBUILD` instruction adds to the image a trigger instruction to be executed at a later
/// time, when the image is used as the base for another build. The trigger will be executed in
/// the context of the downstream build, as if it had been inserted immediately after the `FROM`
/// instruction in the downstream `Dockerfile`.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#onbuild
pub struct Onbuild(Cow<'static, str>);

impl Onbuild {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Onbuild(val.into())
    }
}

impl fmt::Display for Onbuild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ONBUILD {}\n", &self.0)
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

/// The `SHELL` instruction allows the default shell used for the shell form of commands to be
/// overridden.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#shell
pub struct Shell(Cow<'static, str>);

impl Shell {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Shell(val.into())
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SHELL {}\n", &self.0)
    }
}

/// The `STOPSIGNAL` instruction sets the system call signal that will be sent to the container
/// to exit.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#stopsignal
pub struct Stopsignal(Cow<'static, str>);

impl Stopsignal {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Stopsignal(val.into())
    }
}

impl fmt::Display for Stopsignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STOPSIGNAL {}\n", &self.0)
    }
}

/// The `USER` instruction sets the user name (or UID) and optionally the user group (or GID) to
/// use when running the image and for any `RUN`, `CMD` and `ENTRYPOINT` instructions that follow
/// it in the `Dockerfile`.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#user
pub struct User(Cow<'static, str>);

impl User {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        User(val.into())
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "USER {}\n", &self.0)
    }
}

/// The `VOLUME` instruction creates a mount point with the specified name and marks it as holding
/// externally mounted volumes from native host or other containers.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#volume
pub struct Volume(Cow<'static, str>);

impl Volume {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Volume(val.into())
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VOLUME {}\n", &self.0)
    }
}

/// The `WORKDIR` instruction sets the working directory for any `RUN`, `CMD`, `ENTRYPOINT`,
/// `COPY` and `ADD` instructions that follow it in the `Dockerfile`.
///
/// See the docs at https://docs.docker.com/engine/reference/builder/#workdir
pub struct Workdir(Cow<'static, str>);

impl Workdir {
    pub fn new<T: Into<Cow<'static, str>>>(val: T) -> Self {
        Workdir(val.into())
    }
}

impl fmt::Display for Workdir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WORKDIR {}\n", &self.0)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
// Instruction Enum //////////////////////////////////////////////////////////////////////////////

/// An enum representing all of the different types of Dockerfile instructions available.
///
/// This should be the full list according to the latest
/// [Dockerfile spec here](https://docs.docker.com/engine/reference/builder/). If you notice any
/// missing instructions, please
/// [open an issue here](https://github.com/thedodd/dockerfile/issues/new).
pub enum Instruction {
    Add(Add),
    Arg(Arg),
    Cmd(Cmd),
    Copy(Copy),
    Directive(Directive),
    Entrypoint(Entrypoint),
    Env(Env),
    Expose(Expose),
    From(From),
    Healthcheck(Healthcheck),
    Label(Label),
    Onbuild(Onbuild),
    Run(Run),
    Shell(Shell),
    Stopsignal(Stopsignal),
    User(User),
    Volume(Volume),
    Workdir(Workdir),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Add(inst) => write!(f, "{}", inst),
            Instruction::Arg(inst) => write!(f, "{}", inst),
            Instruction::Cmd(inst) => write!(f, "{}", inst),
            Instruction::Copy(inst) => write!(f, "{}", inst),
            Instruction::Directive(inst) => write!(f, "{}", inst),
            Instruction::Entrypoint(inst) => write!(f, "{}", inst),
            Instruction::Env(inst) => write!(f, "{}", inst),
            Instruction::Expose(inst) => write!(f, "{}", inst),
            Instruction::From(inst) => write!(f, "{}", inst),
            Instruction::Healthcheck(inst) => write!(f, "{}", inst),
            Instruction::Label(inst) => write!(f, "{}", inst),
            Instruction::Onbuild(inst) => write!(f, "{}", inst),
            Instruction::Run(inst) => write!(f, "{}", inst),
            Instruction::Shell(inst) => write!(f, "{}", inst),
            Instruction::Stopsignal(inst) => write!(f, "{}", inst),
            Instruction::User(inst) => write!(f, "{}", inst),
            Instruction::Volume(inst) => write!(f, "{}", inst),
            Instruction::Workdir(inst) => write!(f, "{}", inst),
        }
    }
}

impl convert::From<Add> for Instruction {
    fn from(inst: Add) -> Self {

        Instruction::Add(inst)
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
impl convert::From<Env> for Instruction {
    fn from(inst: Env) -> Self {

        Instruction::Env(inst)
    }
}
impl convert::From<Expose> for Instruction {
    fn from(inst: Expose) -> Self {

        Instruction::Expose(inst)
    }
}
impl convert::From<From> for Instruction {
    fn from(inst: From) -> Self {

        Instruction::From(inst)
    }
}
impl convert::From<Healthcheck> for Instruction {
    fn from(inst: Healthcheck) -> Self {

        Instruction::Healthcheck(inst)
    }
}
impl convert::From<Label> for Instruction {
    fn from(inst: Label) -> Self {

        Instruction::Label(inst)
    }
}
impl convert::From<Onbuild> for Instruction {
    fn from(inst: Onbuild) -> Self {

        Instruction::Onbuild(inst)
    }
}
impl convert::From<Run> for Instruction {
    fn from(inst: Run) -> Self {

        Instruction::Run(inst)
    }
}
impl convert::From<Shell> for Instruction {
    fn from(inst: Shell) -> Self {

        Instruction::Shell(inst)
    }
}
impl convert::From<Stopsignal> for Instruction {
    fn from(inst: Stopsignal) -> Self {

        Instruction::Stopsignal(inst)
    }
}
impl convert::From<User> for Instruction {
    fn from(inst: User) -> Self {

        Instruction::User(inst)
    }
}
impl convert::From<Volume> for Instruction {
    fn from(inst: Volume) -> Self {

        Instruction::Volume(inst)
    }
}
impl convert::From<Workdir> for Instruction {
    fn from(inst: Workdir) -> Self {

        Instruction::Workdir(inst)
    }
}
