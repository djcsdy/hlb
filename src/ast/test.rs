use crate::ast::{
    BasicLiteral, BindClause, CallExpression, CallStatement, FunctionDeclaration, FunctionLiteral,
    FunctionSignature, Kind, Module, WithClause,
};

#[test]
fn serialize_module() {
    assert_eq!(
        Module(vec![
            FunctionDeclaration {
                signature: FunctionSignature {
                    return_type: Kind::Filesystem,
                    name: "default".parse().unwrap(),
                    parameters: vec![].into(),
                    effects: None,
                },
                body: vec![
                    CallStatement {
                        name: "scratch".parse().unwrap(),
                        arguments: vec![],
                        with_clause: None,
                        bind_clause: None,
                    }
                    .into(),
                    CallStatement {
                        name: "run".parse().unwrap(),
                        arguments: vec![
                            BasicLiteral::from("/usr/local/bin/docker").into(),
                            BasicLiteral::from("version").into(),
                        ],
                        with_clause: Some(WithClause(
                            FunctionLiteral {
                                kind: Kind::Options,
                                body: vec![
                                    CallStatement {
                                        name: "ignoreCache".parse().unwrap(),
                                        arguments: vec![].into(),
                                        with_clause: None,
                                        bind_clause: None
                                    }
                                    .into(),
                                    CallStatement {
                                        name: "mountDocker".parse().unwrap(),
                                        arguments: vec![].into(),
                                        with_clause: None,
                                        bind_clause: None,
                                    }
                                    .into()
                                ]
                                .into()
                            }
                            .into()
                        )),
                        bind_clause: None,
                    }
                    .into()
                ]
                .into()
            }
            .into(),
            FunctionDeclaration {
                signature: FunctionSignature {
                    return_type: Kind::Filesystem,
                    name: "buildDockerCli".parse().unwrap(),
                    parameters: vec![].into(),
                    effects: None,
                },
                body: vec![
                    CallStatement {
                        name: "image".parse().unwrap(),
                        arguments: vec![BasicLiteral::from("golang:alpine").into()],
                        with_clause: Some(WithClause(
                            FunctionLiteral {
                                kind: Kind::Options,
                                body: vec![
                                    CallStatement {
                                        name: "resolve".parse().unwrap(),
                                        arguments: vec![],
                                        with_clause: None,
                                        bind_clause: None,
                                    }
                                    .into()
                                ]
                                .into(),
                            }
                            .into()
                        )),
                        bind_clause: None,
                    }
                    .into(),
                    CallStatement {
                        name: "run".parse().unwrap(),
                        arguments: vec![
                            BasicLiteral::from("apk add -U git bash coreutils gcc musl-dev").into()
                        ],
                        with_clause: None,
                        bind_clause: None,
                    }
                    .into(),
                    CallStatement {
                        name: "env".parse().unwrap(),
                        arguments: vec![
                            BasicLiteral::from("CGO_ENABLED").into(),
                            BasicLiteral::from("0").into()
                        ],
                        with_clause: None,
                        bind_clause: None,
                    }
                    .into(),
                    CallStatement {
                        name: "env".parse().unwrap(),
                        arguments: vec![
                            BasicLiteral::from("DISABLE_WARN_OUTSIDE_CONTAINER").into(),
                            BasicLiteral::from("1").into()
                        ],
                        with_clause: None,
                        bind_clause: None,
                    }
                    .into(),
                    CallStatement {
                        name: "run".parse().unwrap(),
                        arguments: vec![BasicLiteral::from("./scripts/build/binary").into(),],
                        with_clause: Some(WithClause(
                            FunctionLiteral {
                                kind: Kind::Options,
                                body: vec![
                                    CallStatement {
                                        name: "dir".parse().unwrap(),
                                        arguments: vec![
                                            BasicLiteral::from("/go/src/github.com/docker/cli")
                                                .into(),
                                        ],
                                        with_clause: None,
                                        bind_clause: None,
                                    }
                                    .into(),
                                    CallStatement {
                                        name: "mount".parse().unwrap(),
                                        arguments: vec![
                                            FunctionLiteral {
                                                kind: Kind::Filesystem,
                                                body: vec![
                                                    CallStatement {
                                                        name: "git".parse().unwrap(),
                                                        arguments: vec![
                                                            BasicLiteral::from(
                                                                "https://github.com/docker/cli.git"
                                                            )
                                                            .into(),
                                                            BasicLiteral::from("v19.03.8").into(),
                                                        ],
                                                        with_clause: None,
                                                        bind_clause: None,
                                                    }
                                                    .into()
                                                ]
                                                .into()
                                            }
                                            .into(),
                                            BasicLiteral::from("/go/src/github.com/docker/cli")
                                                .into()
                                        ],
                                        with_clause: None,
                                        bind_clause: None,
                                    }
                                    .into(),
                                    CallStatement {
                                        name: "mount".parse().unwrap(),
                                        arguments: vec![
                                            CallExpression {
                                                name: "scratch".parse().unwrap(),
                                                arguments: vec![].into()
                                            }
                                            .into(),
                                            BasicLiteral::from(
                                                "/go/src/github.com/docker/cli/build"
                                            )
                                            .into()
                                        ],
                                        with_clause: None,
                                        bind_clause: Some(BindClause::Identifier(
                                            "dockerCli".parse().unwrap()
                                        )),
                                    }
                                    .into()
                                ]
                                .into()
                            }
                            .into()
                        )),
                        bind_clause: None,
                    }
                    .into()
                ]
                .into(),
            }
            .into()
        ])
        .to_string(),
        concat!(
            "fs default() {\n",
            "scratch;\n",
            "run \"/usr/local/bin/docker\" \"version\" with option {\n",
            "ignoreCache;\n",
            "mountDocker;\n",
            "};\n",
            "}\n",
            "\n",
            "fs buildDockerCli() {\n",
            "image \"golang:alpine\" with option {\n",
            "resolve;\n",
            "};\n",
            "run \"apk add -U git bash coreutils gcc musl-dev\";\n",
            "env \"CGO_ENABLED\" \"0\";\n",
            "env \"DISABLE_WARN_OUTSIDE_CONTAINER\" \"1\";\n",
            "run \"./scripts/build/binary\" with option {\n",
            "dir \"/go/src/github.com/docker/cli\";\n",
            "mount fs {\n",
            "git \"https://github.com/docker/cli.git\" \"v19.03.8\";\n",
            "} \"/go/src/github.com/docker/cli\";\n",
            "mount scratch() \"/go/src/github.com/docker/cli/build\" as dockerCli;\n",
            "};\n",
            "}\n"
        )
    )
}
