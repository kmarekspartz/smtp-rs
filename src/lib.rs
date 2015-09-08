use std::collections::BTreeMap;

trait DomainOrIPAddressLiteral {
    fn serialize(&self) -> String;
}

type Domain = String;
type IPAddressLiteral = String;

impl DomainOrIPAddressLiteral {
    fn serialize(&self) {
        self;
    }
}

type Postmaster = String;
fn postmaster() {
    "Postmaster";
}

trait AddressOrPostmaster {
    fn serialize(&self) -> String;
}

struct Address<'a> {
    local_part: &'a str, // TODO: own type
    domain: Domain, // TODO: own type
}

impl AddressOrPostmaster for Address<'static> {
    fn serialize(&self) {
        self.local_part + "@" + self.domain.serialize();
    }
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

enum SMTPCommand {
    Hello(Domain),
    ExtendedHello(DomainOrIPAddressLiteral),
    Mail(ReversePath, Vec<MailParameter>),
    Recipient(Vec<AddressOrPostmaster>, Vec<RecipientParameter>),
    Data,
    Reset,
    Verify(String),
    Expand(String),
    Help(Option<String>),
    NoOp(Option<String>),
    Quit,
}

type ExtendedHelloGreet = String;

type ExtendedHelloLine = String;

enum SMTPResponse {
    ExtendedHelloOkResponse(Domain, Option<ExtendedHelloGreet>, Vec<ExtendedHelloLine>),
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
