use clap::Parser;

use crate::cli::{AuthSubcommand, AuthTokenSubcommand, Cli, Command, KeysSubcommand};

#[test]
fn parses_auth_token_get_set_clear() {
    let get = Cli::try_parse_from(["monk", "auth", "token", "get"]).expect("get should parse");
    assert_auth_token_subcommand(get, AuthTokenSubcommand::Get);

    let clear = Cli::try_parse_from(["monk", "auth", "token", "clear"]).expect("clear should parse");
    assert_auth_token_subcommand(clear, AuthTokenSubcommand::Clear);
}

#[test]
fn parses_auth_token_set_with_positional_token() {
    let cli = Cli::try_parse_from(["monk", "auth", "token", "set", "jwt-test-value"]).expect("set should parse");

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
    let provision = Cli::try_parse_from([
        "monk",
        "auth",
        "provision",
        "--tenant",
        "acme",
        "--username",
        "machine_root",
        "--public-key",
        "@machine.pub",
    ])
    .expect("provision should parse");

    match provision.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Provision(args) => {
                assert_eq!(args.tenant.as_deref(), Some("acme"));
                assert_eq!(args.username.as_deref(), Some("machine_root"));
                assert_eq!(args.public_key.as_deref(), Some("@machine.pub"));
            }
            other => panic!("expected provision command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }

    let verify = Cli::try_parse_from([
        "monk",
        "auth",
        "verify",
        "--tenant",
        "acme",
        "--challenge-id",
        "challenge-1",
        "--signature",
        "@sig.txt",
    ])
    .expect("verify should parse");

    match verify.command {
        Command::Auth(auth) => match auth.command {
            AuthSubcommand::Verify(args) => {
                assert_eq!(args.challenge_id.as_deref(), Some("challenge-1"));
                assert_eq!(args.signature.as_deref(), Some("@sig.txt"));
            }
            other => panic!("expected verify command, got {other:?}"),
        },
        other => panic!("expected auth command, got {other:?}"),
    }
}

#[test]
fn parses_keys_commands() {
    let create = Cli::try_parse_from([
        "monk",
        "keys",
        "create",
        "--user-id",
        "user-1",
        "--public-key",
        "@machine.pub",
    ])
    .expect("keys create should parse");

    match create.command {
        Command::Keys(keys) => match keys.command {
            KeysSubcommand::Create(args) => {
                assert_eq!(args.user_id.as_deref(), Some("user-1"));
                assert_eq!(args.public_key.as_deref(), Some("@machine.pub"));
            }
            other => panic!("expected keys create command, got {other:?}"),
        },
        other => panic!("expected keys command, got {other:?}"),
    }

    let rotate = Cli::try_parse_from([
        "monk",
        "keys",
        "rotate",
        "--key-id",
        "key-1",
        "--new-public-key",
        "@next.pub",
        "--revoke-old-after-seconds",
        "120",
    ])
    .expect("keys rotate should parse");

    match rotate.command {
        Command::Keys(keys) => match keys.command {
            KeysSubcommand::Rotate(args) => {
                assert_eq!(args.key_id.as_deref(), Some("key-1"));
                assert_eq!(args.new_public_key.as_deref(), Some("@next.pub"));
                assert_eq!(args.revoke_old_after_seconds, Some(120));
            }
            other => panic!("expected keys rotate command, got {other:?}"),
        },
        other => panic!("expected keys command, got {other:?}"),
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
