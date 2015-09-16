use std::collections::BTreeMap;
use std::io::prelude::*;

enum DomainOrIPAddressLiteral {
    ADomain(Domain),
    IPAddressLiteral(String), // TODO: Better type
}

// TODO: Better type:
type Domain = String;

// TODO: Better type:
type Postmaster = String;

fn postmaster() -> String {
    "Postmaster".to_string()
}

enum AddressOrPostmaster {
    AnAddress(Address),
    Postmaster,
}

struct Address {
    local_part: String, // TODO: Better type
    domain: Domain,
}

enum OneOrMore<T> {
    One(T),
    More(T,Box<OneOrMore<T>>),
}

enum ProtocolExtension {
    EightBitMIME,
    // TODO: List more
}

struct Envelope {
    originator: Address,
    recipients: OneOrMore<Address>,
    protocol_extensions: Vec<ProtocolExtension>,
}

struct Header {
    fields: BTreeMap<String, String>, // TODO: Case-insensitivity
}

struct Content {
    header: Header,
    body: String, // TODO: Better type
}

struct MailObject {
    envelope: Envelope,
    content: Content,
}

 // TODO: Better types:
type ReversePath = String;
type MailParameter = String;
type RecipientParameter = String;

enum SMTPCommand {
    Hello(Domain),
    ExtendedHello(DomainOrIPAddressLiteral),
    Mail(ReversePath, Vec<MailParameter>),
    Recipient(Vec<AddressOrPostmaster>, Vec<RecipientParameter>),
    Data,
    Reset,
    Verify(String), // TODO: Better type
    Expand(String), // TODO: Better type
    Help(Option<String>), // TODO: Better type
    NoOp(Option<String>), // TODO: Better type
    Quit,
}

fn serialize_helo(domain: Domain) -> String {
    format!("HELO {}", domain)
}

fn serialize_ehlo(domain_or_ip_address_literal: DomainOrIPAddressLiteral) -> String {
    let host = match domain_or_ip_address_literal {
        DomainOrIPAddressLiteral::ADomain(domain) => domain,
        DomainOrIPAddressLiteral::IPAddressLiteral(ip) => ip,
    };
    format!("EHLO {}", host)

}

type ExtendedHelloGreet = String; // TODO: Better type
type ExtendedHelloLine = String; // TODO: Better type

enum SMTPResponse {
    ExtendedHelloOkResponse(Domain, Option<ExtendedHelloGreet>, Vec<ExtendedHelloLine>),
}

fn serialize(command: SMTPCommand) -> String {
    match command {
        SMTPCommand::Hello(domain) => serialize_helo(domain),
        SMTPCommand::ExtendedHello(domain_or_ip_address_literal) => serialize_ehlo(domain_or_ip_address_literal),
        _ => "OOPS!".to_string(),
    }
}

fn send_command(stream: &mut Write, command: SMTPCommand) -> std::io::Result<usize> {
    stream.write(serialize(command).as_bytes())
}


#[test]
fn it_works() {
    let mail_object = MailObject {
        envelope: Envelope {
            originator: Address {
                local_part: "kyle.marek.spartz".to_string(),
                domain: "gmail.com".to_string(),
            },
            recipients: OneOrMore::More(Address {
                    local_part: "zeckalpha".to_string(),
                    domain: "gmail.com".to_string(),
                }, Box::new(OneOrMore::One(Address {
                    local_part: "kyle".to_string(),
                    domain: "marek-spartz.org".to_string(),
                }))
            ),
            protocol_extensions: vec![
                ProtocolExtension::EightBitMIME,
            ]
        },
        content: Content {
            header: Header {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert("orig-date".to_string(), "...".to_string());
                    fields.insert("from".to_string(), "kyle.marek.spartz@gmail.com".to_string());
                    fields
                }
            },
            body: "LOL".to_string()
        }
    };
    assert_eq!("LOL", mail_object.content.body);
}

#[test]
fn send_command_sends_hello() {
    use std::fs::File;

    let mut write_stream = File::create("helo_output").ok().unwrap();
    let mut content = String::new();
    let hello = SMTPCommand::Hello("mail.google.com".to_string());
    assert_eq!(20, send_command(&mut write_stream, hello).ok().unwrap());
    write_stream.flush();
    let mut read_stream = File::open("helo_output").ok().unwrap();
    assert_eq!(20, read_stream.read_to_string(&mut content).ok().unwrap());
    assert_eq!("HELO mail.google.com", content);
}

#[test]
fn hello_serializes() {
    let hello_google = SMTPCommand::Hello("mail.google.com".to_string());
    let hello_yahoo = SMTPCommand::Hello("mail.yahoo.com".to_string());
    assert_eq!("HELO mail.google.com", serialize(hello_google));
    assert_eq!("HELO mail.yahoo.com", serialize(hello_yahoo));
}

#[test]
fn extended_hello_serializes() {
    let extended_hello_google = SMTPCommand::ExtendedHello(DomainOrIPAddressLiteral::ADomain("mail.google.com".to_string()));
    let extended_hello_yahoo = SMTPCommand::ExtendedHello(DomainOrIPAddressLiteral::ADomain("mail.yahoo.com".to_string()));
    assert_eq!("EHLO mail.google.com", serialize(extended_hello_google));
    assert_eq!("EHLO mail.yahoo.com", serialize(extended_hello_yahoo));
}
