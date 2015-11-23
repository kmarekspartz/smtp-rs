use std::collections::BTreeMap;
use std::io::prelude::*;

enum DomainOrIPAddressLiteral<'a> {
    ADomain(Domain<'a>),
    IPAddressLiteral(&'a str), // TODO: Better type
}

// TODO: Better type:
type Domain<'a> = &'a str;

// TODO: Better type:
type Postmaster<'a> = &'a str;

fn postmaster<'a>() -> &'a str {
    "Postmaster"
}

enum AddressOrPostmaster<'a> {
    AnAddress(Address<'a>),
    Postmaster,
}

struct Address<'a> {
    local_part: &'a str, // TODO: Better type
    domain: Domain<'a>,
}

enum OneOrMore<T> {
    One(T),
    More(T, Box<OneOrMore<T>>),
}

enum ProtocolExtension {
    EightBitMIME, // TODO: List more
}

struct Envelope<'a> {
    originator: Address<'a>,
    recipients: OneOrMore<Address<'a>>,
    protocol_extensions: Vec<ProtocolExtension>,
}

struct Header<'a> {
    fields: BTreeMap<&'a str, &'a str>, // TODO: Case-insensitivity
}

struct Content<'a> {
    header: Header<'a>,
    body: &'a str, // TODO: Better type
}

struct MailObject<'a> {
    envelope: Envelope<'a>,
    content: Content<'a>,
}

// TODO: Better types:
type ReversePath<'a> = &'a str;
type MailParameter<'a> = &'a str;
type RecipientParameter<'a> = &'a str;

enum SMTPCommand<'a> {
    Hello(Domain<'a>),
    ExtendedHello(DomainOrIPAddressLiteral<'a>),
    Mail(ReversePath<'a>, Vec<MailParameter<'a>>),
    Recipient(Vec<AddressOrPostmaster<'a>>, Vec<RecipientParameter<'a>>),
    Data,
    Reset,
    Verify(&'a str), // TODO: Better type
    Expand(&'a str), // TODO: Better type
    Help(Option<&'a str>), // TODO: Better type
    NoOp(Option<&'a str>), // TODO: Better type
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

type ExtendedHelloGreet<'a> = &'a str; // TODO: Better type
type ExtendedHelloLine<'a> = &'a str; // TODO: Better type

enum SMTPResponse<'a> {
    ExtendedHelloOkResponse(Domain<'a>, Option<ExtendedHelloGreet<'a>>, Vec<ExtendedHelloLine<'a>>),
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
                local_part: "kyle.marek.spartz",
                domain: "gmail.com",
            },
            recipients: OneOrMore::More(Address {
                                            local_part: "zeckalpha",
                                            domain: "gmail.com",
                                        },
                                        Box::new(OneOrMore::One(Address {
                                            local_part: "kyle",
                                            domain: "marek-spartz.org",
                                        }))),
            protocol_extensions: vec![
                ProtocolExtension::EightBitMIME,
            ],
        },
        content: Content {
            header: Header {
                fields: {
                    let mut fields = BTreeMap::new();
                    fields.insert("orig-date", "...");
                    fields.insert("from", "kyle.marek.spartz@gmail.com");
                    fields
                },
            },
            body: "LOL",
        },
    };
    assert_eq!("LOL", mail_object.content.body);
}

#[test]
fn send_command_sends_hello() {
    extern crate tempfile;
    use std::io::{Write, Read, Seek, SeekFrom};

    let mut temp_file = tempfile::TempFile::new().unwrap();
    let mut content = String::new();
    let hello = SMTPCommand::Hello("mail.google.com");

    assert_eq!(20, send_command(&mut temp_file, hello).ok().unwrap());

    temp_file.flush();
    temp_file.seek(SeekFrom::Start(0)).unwrap();

    assert_eq!(20, temp_file.read_to_string(&mut content).ok().unwrap());
    assert_eq!("HELO mail.google.com", content);
}

#[test]
fn hello_serializes() {
    let hello_google = SMTPCommand::Hello("mail.google.com");
    let hello_yahoo = SMTPCommand::Hello("mail.yahoo.com");

    assert_eq!("HELO mail.google.com", serialize(hello_google));
    assert_eq!("HELO mail.yahoo.com", serialize(hello_yahoo));
}

#[test]
fn extended_hello_serializes() {
    let extended_hello_google = SMTPCommand::ExtendedHello(DomainOrIPAddressLiteral::ADomain("mail.google.com"));
    let extended_hello_yahoo = SMTPCommand::ExtendedHello(DomainOrIPAddressLiteral::ADomain("mail.yahoo.com"));

    assert_eq!("EHLO mail.google.com", serialize(extended_hello_google));
    assert_eq!("EHLO mail.yahoo.com", serialize(extended_hello_yahoo));
}
