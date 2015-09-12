use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;

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

fn helo(stream: Write) -> std::io::Result<usize> {
  let mut helo_string = "HELO";
  stream.write(helo_string);
}

type ExtendedHelloGreet = String;

type ExtendedHelloLine = String;

enum SMTPResponse<'a> {
    ExtendedHelloOkResponse(Domain<'a>, Option<ExtendedHelloGreet>, Vec<ExtendedHelloLine>),
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
fn helo_writes() {
    let mut stream = try!(File::create("helo_output"));
    let mut content = String::new();
    helo(stream);
    stream.read_to_string(&mut content);
    assert_eq!("HELO", content);
}
