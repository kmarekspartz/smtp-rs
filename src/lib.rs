use std::collections::BTreeMap;
use std::io::prelude::*;

enum DomainOrIPAddressLiteral<'a> {
    ADomain(Domain<'a>),
    IPAddressLiteral(&'a str),
}

// TODO: Better type:
type Domain<'a> = &'a str;

type Postmaster = String;
fn postmaster() {
    "Postmaster";
}

enum AddressOrPostmaster<'a> {
    AnAddress(Address<'a>),
    Postmaster,
}

struct Address<'a> {
    local_part: &'a str, // TODO: own type
    domain: Domain<'a>,
}

enum OneOrMore<T> {
    One(T),
    More(T,Box<OneOrMore<T>>),
}

enum ProtocolExtension {
    EightBitMIME,
}

struct Envelope<'a> {
    originator: Address<'a>,
    recipients: OneOrMore<Address<'a>>,
    protocol_extensions: Vec<ProtocolExtension>
}

struct Header<'a> {
    fields: BTreeMap<&'a str, &'a str> // Case-insensitivity
}

struct Content<'a> {
    header: Header<'a>,
    body: &'a str, // TODO: own type
}

struct MailObject<'a> {
    envelope: Envelope<'a>,
    content: Content<'a>,
}

type ReversePath = String;
type MailParameter = String;
type RecipientParameter = String;

enum SMTPCommand<'a> {
    Hello(Domain<'a>),
    ExtendedHello(DomainOrIPAddressLiteral<'a>),
    Mail(ReversePath, Vec<MailParameter>),
    Recipient(Vec<AddressOrPostmaster<'a>>, Vec<RecipientParameter>),
    Data,
    Reset,
    Verify(String),
    Expand(String),
    Help(Option<String>),
    NoOp(Option<String>),
    Quit,
}

fn serialize_helo(domain: &str) -> String {
    format!("HELO {}", domain)
}

type ExtendedHelloGreet = String;

type ExtendedHelloLine = String;

enum SMTPResponse<'a> {
    ExtendedHelloOkResponse(Domain<'a>, Option<ExtendedHelloGreet>, Vec<ExtendedHelloLine>),
}

fn serialize(command: SMTPCommand) -> String {
    match command {
        SMTPCommand::Hello(domain) => serialize_helo(domain),
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
                }, Box::new(OneOrMore::One(Address {
                    local_part: "kyle",
                    domain: "marek-spartz.org",
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
                    fields.insert("orig-date", "...");
                    fields.insert("from", "kyle.marek.spartz@gmail.com");
                    fields
                }
            },
            body: "LOL"
        }
    };
    assert_eq!("LOL", mail_object.content.body);
}

#[test]
fn send_command_sends_hello() {
    use std::fs::File;

    let mut write_stream = File::create("helo_output").ok().unwrap();
    let mut content = String::new();
    let hello = SMTPCommand::Hello("mail.google.com");
    assert_eq!(20, send_command(&mut write_stream, hello).ok().unwrap());
    write_stream.flush();
    let mut read_stream = File::open("helo_output").ok().unwrap();
    assert_eq!(20, read_stream.read_to_string(&mut content).ok().unwrap());
    assert_eq!("HELO mail.google.com", content);
}

#[test]
fn hello_serializes() {
    let hello_google = SMTPCommand::Hello("mail.google.com");
    let hello_yahoo = SMTPCommand::Hello("mail.yahoo.com");
    assert_eq!("HELO mail.google.com", serialize(hello_google));
    assert_eq!("HELO mail.yahoo.com", serialize(hello_yahoo));
}
