use clap::Parser;

use crate::cli::{
    AuthSubcommand, AuthTokenSubcommand, Cli, Command, ConfigCommand, ConfigSubcommand,
    DataSubcommand, DoctorCommand, KeysSubcommand, LlmFactorySubcommand, LlmRoomSubcommand,
    LlmSubcommand, TuiCommand, UpdateCommand, UserMachineKeysSubcommand,
};

#[test]
fn parses_global_config_flag() {
    let cli = Cli::try_parse_from(["abbot", "--config", "staging", "health"])
        .expect("global config flag should parse");

    assert_eq!(cli.globals.config.as_deref(), Some("staging"));
    assert!(matches!(cli.command, Command::Health));
}

#[test]
fn parses_auth_token_get_set_clear() {
    let get = Cli::try_parse_from(["abbot", "auth", "token", "get"]).expect("get should parse");
    assert_auth_token_subcommand(get, AuthTokenSubcommand::Get);

    let clear =
        Cli::try_parse_from(["abbot", "auth", "token", "clear"]).expect("clear should parse");
    assert_auth_token_subcommand(clear, AuthTokenSubcommand::Clear);
}

#[test]
fn parses_auth_token_set_with_positional_token() {
    let cli = Cli::try_parse_from(["abbot", "auth", "token", "set", "jwt-test-value"])
        .expect("set should parse");

    match cli.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Token(token) => match token.command {
                AuthTokenSubcommand::Set(args) => {
                    assert_eq!(args.token, "jwt-test-value");
                }
                other => panic!("expected token set command, got {other:?}"),
            },
            other => panic!("expected auth token command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }
}

#[test]
fn parses_machine_auth_commands() {
    let connect = Cli::try_parse_from([
        "abbot",
        "auth",
        "machine",
        "connect",
        "--tenant",
        "acme",
        "--username",
        "machine_root",
        "--key",
        "@~/.config/secrets/machine.key",
    ])
    .expect("machine connect should parse");

    match connect.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Machine(machine) => match machine.command {
                crate::cli::AuthMachineSubcommand::Connect(args) => {
                    assert_eq!(args.tenant.as_deref(), Some("acme"));
                    assert_eq!(args.username.as_deref(), Some("machine_root"));
                    assert_eq!(args.key.as_deref(), Some("@~/.config/secrets/machine.key"));
                }
            },
            other => panic!("expected machine command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }

    let provision = Cli::try_parse_from([
        "abbot",
        "auth",
        "provision",
        "--tenant",
        "acme",
        "--username",
        "machine_root",
        "--public-key",
        "@machine.pub",
        "--save-private-key-path",
        "~/.config/secrets/machine.key",
    ])
    .expect("provision should parse");

    match provision.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Provision(args) => {
                assert_eq!(args.tenant.as_deref(), Some("acme"));
                assert_eq!(args.username.as_deref(), Some("machine_root"));
                assert_eq!(args.public_key.as_deref(), Some("@machine.pub"));
                assert_eq!(
                    args.save_private_key_path.as_deref(),
                    Some("~/.config/secrets/machine.key")
                );
            }
            other => panic!("expected provision command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }

    let invite_provision = Cli::try_parse_from([
        "abbot",
        "auth",
        "provision",
        "--tenant",
        "acme",
        "--username",
        "builder_2",
        "--invite-code",
        "invite-code-1",
        "--public-key",
        "@machine.pub",
    ])
    .expect("invite provision should parse");

    match invite_provision.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Provision(args) => {
                assert_eq!(args.invite_code.as_deref(), Some("invite-code-1"));
                assert_eq!(args.username.as_deref(), Some("builder_2"));
            }
            other => panic!("expected provision command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }

    let verify = Cli::try_parse_from([
        "abbot",
        "auth",
        "verify",
        "--tenant",
        "acme",
        "--challenge-id",
        "challenge-1",
        "--signature",
        "@sig.txt",
        "--save-public-key-path",
        "~/.config/secrets/machine.pub",
    ])
    .expect("verify should parse");

    match verify.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Verify(args) => {
                assert_eq!(args.challenge_id.as_deref(), Some("challenge-1"));
                assert_eq!(args.signature.as_deref(), Some("@sig.txt"));
                assert_eq!(
                    args.save_public_key_path.as_deref(),
                    Some("~/.config/secrets/machine.pub")
                );
            }
            other => panic!("expected verify command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }
}

#[test]
fn parses_invited_human_and_user_invite_commands() {
    let register = Cli::try_parse_from([
        "abbot",
        "auth",
        "register",
        "--tenant",
        "acme",
        "--username",
        "alice",
        "--invite-code",
        "invite-code-1",
        "--email",
        "alice@example.com",
        "--password",
        "secret-pass",
    ])
    .expect("invite register should parse");

    match register.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Register(args) => {
                assert_eq!(args.invite_code.as_deref(), Some("invite-code-1"));
                assert_eq!(args.email.as_deref(), Some("alice@example.com"));
            }
            other => panic!("expected register command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }

    let invite = Cli::try_parse_from([
        "abbot",
        "user",
        "invite",
        "--username",
        "builder_2",
        "--invite-type",
        "machine",
        "--access",
        "edit",
        "--access-read",
        "rooms",
        "--access-read",
        "jobs",
        "--expires-in",
        "3600",
    ])
    .expect("user invite should parse");

    match invite.command {
        Command::User(user) => match user.command {
            crate::cli::UserSubcommand::Invite(args) => {
                assert_eq!(args.username.as_deref(), Some("builder_2"));
                assert_eq!(args.invite_type.as_deref(), Some("machine"));
                assert_eq!(args.access.as_deref(), Some("edit"));
                assert_eq!(
                    args.access_read,
                    vec!["rooms".to_string(), "jobs".to_string()]
                );
                assert_eq!(args.expires_in, Some(3600));
            }
            other => panic!("expected user invite command, got {other:?}"),
        },
        other => panic!("expected user command, got {other:?}"),
    }

    let invalid_user_create = Cli::try_parse_from([
        "abbot", "user", "create", "--name", "anon", "--auth", "anon", "--access", "user",
    ]);
    assert!(
        invalid_user_create.is_err(),
        "invalid access should fail clap parsing"
    );
}

#[test]
fn parses_data_list_with_limit() {
    let cli = Cli::try_parse_from(["abbot", "data", "--limit", "5", "list", "rooms"])
        .expect("data list with limit should parse");

    match cli.command {
        Command::Data(data) => {
            assert_eq!(data.options.limit, Some(5));
            match data.command {
                DataSubcommand::List(arg) => assert_eq!(arg.model, "rooms"),
                other => panic!("expected data list command, got {other:?}"),
            }
        }
        other => panic!("expected data command, got {other:?}"),
    }
}

#[test]
fn parses_keys_commands() {
    let create = Cli::try_parse_from([
        "abbot",
        "keys",
        "create",
        "--name",
        "CI runner",
        "--expires-at",
        "2026-12-31T23:59:59Z",
    ])
    .expect("keys create should parse");

    match create.command {
        Command::Keys(keys) => match keys.command {
            KeysSubcommand::Create(args) => {
                assert_eq!(args.name.as_deref(), Some("CI runner"));
                assert_eq!(args.expires_at.as_deref(), Some("2026-12-31T23:59:59Z"));
            }
            other => panic!("expected keys create command, got {other:?}"),
        },
        other => panic!("expected keys command, got {other:?}"),
    }

    let revoke_all =
        Cli::try_parse_from(["abbot", "keys", "revoke-all"]).expect("keys revoke-all should parse");

    match revoke_all.command {
        Command::Keys(keys) => match keys.command {
            KeysSubcommand::RevokeAll(_) => {}
            other => panic!("expected keys revoke-all command, got {other:?}"),
        },
        other => panic!("expected keys command, got {other:?}"),
    }
}

#[test]
fn parses_top_level_tui_command() {
    let cli = Cli::try_parse_from(["abbot", "tui"]).expect("tui should parse");

    match cli.command {
        Command::Tui(TuiCommand {}) => {}
        other => panic!("expected tui command, got {other:?}"),
    }
}

#[test]
fn parses_top_level_config_and_doctor_commands() {
    let config = Cli::try_parse_from(["abbot", "config"]).expect("config should parse");
    match config.command {
        Command::Config(ConfigCommand { command: None }) => {}
        other => panic!("expected config command, got {other:?}"),
    }

    let doctor = Cli::try_parse_from(["abbot", "doctor"]).expect("doctor should parse");
    match doctor.command {
        Command::Doctor(DoctorCommand {}) => {}
        other => panic!("expected doctor command, got {other:?}"),
    }
}

#[test]
fn parses_config_management_commands() {
    let create = Cli::try_parse_from([
        "abbot",
        "config",
        "create",
        "staging",
        "https://example.com",
    ])
    .expect("config create should parse");
    match create.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::Create(args)),
        }) => {
            assert_eq!(args.name, "staging");
            assert_eq!(args.url.as_deref(), Some("https://example.com"));
        }
        other => panic!("expected config create command, got {other:?}"),
    }

    let use_profile = Cli::try_parse_from(["abbot", "config", "use", "staging"])
        .expect("config use should parse");
    match use_profile.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::Use(args)),
        }) => assert_eq!(args.name, "staging"),
        other => panic!("expected config use command, got {other:?}"),
    }

    let list = Cli::try_parse_from(["abbot", "config", "list"]).expect("config list should parse");
    match list.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::List),
        }) => {}
        other => panic!("expected config list command, got {other:?}"),
    }

    let show =
        Cli::try_parse_from(["abbot", "config", "show", "prod"]).expect("config show should parse");
    match show.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::Show(args)),
        }) => assert_eq!(args.name, "prod"),
        other => panic!("expected config show command, got {other:?}"),
    }

    let set = Cli::try_parse_from([
        "abbot",
        "config",
        "set",
        "prod",
        "base_url",
        "https://api.example.com",
    ])
    .expect("config set should parse");
    match set.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::Set(args)),
        }) => {
            assert_eq!(args.name, "prod");
            assert_eq!(args.key, "base_url");
            assert_eq!(args.value.as_deref(), Some("https://api.example.com"));
            assert!(!args.unset);
        }
        other => panic!("expected config set command, got {other:?}"),
    }

    let unset = Cli::try_parse_from(["abbot", "config", "set", "prod", "token", "--unset"])
        .expect("config set --unset should parse");
    match unset.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::Set(args)),
        }) => {
            assert_eq!(args.name, "prod");
            assert_eq!(args.key, "token");
            assert!(args.value.is_none());
            assert!(args.unset);
        }
        other => panic!("expected config unset command, got {other:?}"),
    }

    let get = Cli::try_parse_from(["abbot", "config", "get", "prod", "token"])
        .expect("config get should parse");
    match get.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::Get(args)),
        }) => {
            assert_eq!(args.name, "prod");
            assert_eq!(args.key, "token");
        }
        other => panic!("expected config get command, got {other:?}"),
    }

    let delete = Cli::try_parse_from(["abbot", "config", "delete", "prod"])
        .expect("config delete should parse");
    match delete.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::Delete(args)),
        }) => assert_eq!(args.name, "prod"),
        other => panic!("expected config delete command, got {other:?}"),
    }

    let doctor =
        Cli::try_parse_from(["abbot", "config", "doctor"]).expect("config doctor should parse");
    match doctor.command {
        Command::Config(ConfigCommand {
            command: Some(ConfigSubcommand::Doctor),
        }) => {}
        other => panic!("expected config doctor command, got {other:?}"),
    }
}

#[test]
fn parses_top_level_update_command() {
    let cli = Cli::try_parse_from(["abbot", "update"]).expect("update should parse");

    match cli.command {
        Command::Update(UpdateCommand {
            version_list: false,
            version: None,
        }) => {}
        other => panic!("expected update command, got {other:?}"),
    }
}

#[test]
fn parses_top_level_update_flags() {
    let list =
        Cli::try_parse_from(["abbot", "update", "--version-list"]).expect("update list parse");
    match list.command {
        Command::Update(UpdateCommand {
            version_list: true,
            version: None,
        }) => {}
        other => panic!("expected update --version-list command, got {other:?}"),
    }

    let version = Cli::try_parse_from(["abbot", "update", "--version", "v1.7.0"])
        .expect("update version parse");
    match version.command {
        Command::Update(UpdateCommand {
            version_list: false,
            version: Some(value),
        }) => assert_eq!(value, "v1.7.0"),
        other => panic!("expected update --version command, got {other:?}"),
    }
}

#[test]
fn parses_command_docs_path() {
    let cli = Cli::try_parse_from(["abbot", "command", "auth", "machine", "connect"])
        .expect("command docs path should parse");

    match cli.command {
        Command::Command(command) => {
            assert_eq!(command.path, vec!["auth", "machine", "connect"]);
        }
        other => panic!("expected command docs command, got {other:?}"),
    }
}

#[test]
fn parses_user_introspect_command() {
    let cli =
        Cli::try_parse_from(["abbot", "user", "introspect"]).expect("introspect should parse");

    match cli.command {
        Command::User(user) => match user.command {
            crate::cli::UserSubcommand::Introspect(_) => {}
            other => panic!("expected user introspect command, got {other:?}"),
        },
        other => panic!("expected user command, got {other:?}"),
    }
}

#[test]
fn parses_user_machine_keys_commands() {
    let create = Cli::try_parse_from([
        "abbot",
        "user",
        "machine-keys",
        "create",
        "--user-id",
        "user-1",
        "--public-key",
        "@machine.pub",
        "--name",
        "CI runner",
        "--expires-at",
        "2026-12-31T23:59:59Z",
    ])
    .expect("user machine-keys create should parse");

    match create.command {
        Command::User(user) => match user.command {
            crate::cli::UserSubcommand::MachineKeys(command) => match command.command {
                UserMachineKeysSubcommand::Create(args) => {
                    assert_eq!(args.user_id.as_deref(), Some("user-1"));
                    assert_eq!(args.public_key.as_deref(), Some("@machine.pub"));
                    assert_eq!(args.name.as_deref(), Some("CI runner"));
                    assert_eq!(args.expires_at.as_deref(), Some("2026-12-31T23:59:59Z"));
                }
                other => panic!("expected user machine-keys create command, got {other:?}"),
            },
            other => panic!("expected user machine-keys command, got {other:?}"),
        },
        other => panic!("expected user command, got {other:?}"),
    }

    let rotate = Cli::try_parse_from([
        "abbot",
        "user",
        "machine-keys",
        "rotate",
        "--key-id",
        "key-1",
        "--new-public-key",
        "@next.pub",
        "--revoke-old-after-seconds",
        "120",
    ])
    .expect("user machine-keys rotate should parse");

    match rotate.command {
        Command::User(user) => match user.command {
            crate::cli::UserSubcommand::MachineKeys(command) => match command.command {
                UserMachineKeysSubcommand::Rotate(args) => {
                    assert_eq!(args.key_id.as_deref(), Some("key-1"));
                    assert_eq!(args.new_public_key.as_deref(), Some("@next.pub"));
                    assert_eq!(args.revoke_old_after_seconds, Some(120));
                }
                other => panic!("expected rotate command, got {other:?}"),
            },
            other => panic!("expected user machine-keys command, got {other:?}"),
        },
        other => panic!("expected user command, got {other:?}"),
    }
}

#[test]
fn parses_llm_room_commands() {
    let message = Cli::try_parse_from(["abbot", "llm", "room", "message", "room_123"])
        .expect("llm room message should parse");

    match message.command {
        Command::Llm(command) => match command.command {
            LlmSubcommand::Room(room) => match room.command {
                LlmRoomSubcommand::Message(arg) => {
                    assert_eq!(arg.id, "room_123");
                }
                other => panic!("expected llm room message command, got {other:?}"),
            },
            other => panic!("expected llm room command, got {other:?}"),
        },
        other => panic!("expected llm command, got {other:?}"),
    }

    let events = Cli::try_parse_from(["abbot", "llm", "room", "events", "room_123", "--follow"])
        .expect("llm room events should parse");

    match events.command {
        Command::Llm(command) => match command.command {
            LlmSubcommand::Room(room) => match room.command {
                LlmRoomSubcommand::Events(args) => {
                    assert_eq!(args.id, "room_123");
                    assert!(args.follow);
                }
                other => panic!("expected llm room events command, got {other:?}"),
            },
            other => panic!("expected llm room command, got {other:?}"),
        },
        other => panic!("expected llm command, got {other:?}"),
    }
}

#[test]
fn parses_llm_factory_commands() {
    let update_stage = Cli::try_parse_from([
        "abbot",
        "llm",
        "factory",
        "update-stage",
        "run_123",
        "stage_456",
    ])
    .expect("llm factory update-stage should parse");

    match update_stage.command {
        Command::Llm(command) => match command.command {
            LlmSubcommand::Factory(factory) => match factory.command {
                LlmFactorySubcommand::UpdateStage(arg) => {
                    assert_eq!(arg.id, "run_123");
                    assert_eq!(arg.stage_id, "stage_456");
                }
                other => panic!("expected llm factory update-stage command, got {other:?}"),
            },
            other => panic!("expected llm factory command, got {other:?}"),
        },
        other => panic!("expected llm command, got {other:?}"),
    }
}

fn assert_auth_token_subcommand(cli: Cli, expected: AuthTokenSubcommand) {
    match cli.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Token(token) => match (token.command, expected) {
                (AuthTokenSubcommand::Get, AuthTokenSubcommand::Get) => {}
                (AuthTokenSubcommand::Clear, AuthTokenSubcommand::Clear) => {}
                (AuthTokenSubcommand::Set(_), AuthTokenSubcommand::Set(_)) => {}
                (actual, expected) => panic!("expected {expected:?}, got {actual:?}"),
            },
            other => panic!("expected auth token command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }
}
