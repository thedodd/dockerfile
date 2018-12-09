#![cfg_attr(feature="docinclude", feature(external_doc))]
#![cfg_attr(feature="docinclude", doc(include="../README.md"))]

mod builder;
mod instructions;

pub use crate::{
    builder::{
        Dockerfile,
        DockerfileBuilder,
    },
    instructions::{
        Add,
        Arg,
        Cmd,
        Copy,
        Directive,
        Entrypoint,
        Env,
        Expose,
        From,
        Healthcheck,
        Label,
        Onbuild,
        Run,
        Shell,
        Stopsignal,
        User,
        Volume,
        Workdir,
        Instruction,
    },
};

//////////////////////////////////////////////////////////////////////////////////////////////////
// Unit Tests ////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dockerfile_generation_simple() {
        assert_eq!(Dockerfile::base("rust:1.30-slim")
            .finish().to_string(),
r##"FROM rust:1.30-slim
"##)
    }

    #[test]
    fn dockerfile_generation_with_directives_and_args() {
        assert_eq!(Dockerfile::base("rust:1.30-slim")
            .push_initial_directive(Directive::new("escape=`"))
            .push_initial_arg(Arg::new("TEST"))
            .push_initial_arg(Arg::new("OTHER=1"))
            .push(Copy::new("/static ./static".to_string()))
            .finish().to_string(),
r##"# escape=`
ARG TEST
ARG OTHER=1
FROM rust:1.30-slim
COPY /static ./static
"##)
    }

    #[test]
    fn dockerfile_readme_example() {
        // Build up a new Dockerfile.
        let dockerfile = Dockerfile::base("rust:${RUST_VERSION}-slim")
            .push_initial_arg(Arg::new("RUST_VERSION=1.31"))
            .push(Copy::new("/static ./static"))
            .push(Cmd::new("echo 'Hello. Goodbye.'"))
            .finish();

        // Write it out as a string.
        let output = dockerfile.to_string();
        assert_eq!(output,
r##"ARG RUST_VERSION=1.31
FROM rust:${RUST_VERSION}-slim
COPY /static ./static
CMD echo 'Hello. Goodbye.'
"##)
    }

    //////////////////////////////////////////////////////////////////////////////////////////////
    // Exhaustive Tests //////////////////////////////////////////////////////////////////////////
    //
    // The tests here are intended to test each of the different instruction types in relative
    // isolation.

    #[test]
    fn dockerfile_with_add() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Add::new("/file ./file"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
ADD /file ./file
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_arg() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Arg::new("VAL=testing"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
ARG VAL=testing
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_cmd() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_copy() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Copy::new("/static ./static"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
COPY /static ./static
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_directive() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Directive::new("escape=`"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
# escape=`
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_entrypoint() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Entrypoint::new("echo"))
            .push(Cmd::new("'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
ENTRYPOINT echo
CMD 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_env() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Env::new("VAL=test"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
ENV VAL=test
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_expose() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Expose::new("80"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
EXPOSE 80
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_from() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(From::new("rust:1.31-slim as other"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
FROM rust:1.31-slim as other
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_healthcheck() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Healthcheck::new("CMD pgrep 1"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
HEALTHCHECK CMD pgrep 1
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_label() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Label::new("maintainer='Anthony J Dodd'"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
LABEL maintainer='Anthony J Dodd'
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_onbuild() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Onbuild::new("RUN echo 'ehlo'"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
ONBUILD RUN echo 'ehlo'
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_run() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Run::new("apt-get update -yy"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
RUN apt-get update -yy
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_shell() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Shell::new(r#"["/bin/sh", "-c"]"#))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
SHELL ["/bin/sh", "-c"]
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_stopsignal() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Stopsignal::new("SIGKILL"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
STOPSIGNAL SIGKILL
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_user() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(User::new("root"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
USER root
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_volume() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Volume::new(r#"["/data"]"#))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
VOLUME ["/data"]
CMD echo 'Hello, world.'
"##)
    }

    #[test]
    fn dockerfile_with_workdir() {
        assert_eq!(Dockerfile::base("rust:1.31-slim")
            .push(Workdir::new("/app"))
            .push(Cmd::new("echo 'Hello, world.'"))
            .finish().to_string(),
r##"FROM rust:1.31-slim
WORKDIR /app
CMD echo 'Hello, world.'
"##)
    }
}
