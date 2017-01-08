use field::Field;

impl Field for Account {
    fn tag(&self) -> u16 {
        return 1;
    }
}
pub struct Account {
    value: char,
}
impl Field for AdvId {
    fn tag(&self) -> u16 {
        return 2;
    }
}
pub struct AdvId {
    value: u16,
}
impl Field for AdvRefID {
    fn tag(&self) -> u16 {
        return 3;
    }
}
pub struct AdvRefID {
    value: u16,
}
impl Field for AdvSide {
    fn tag(&self) -> u16 {
        return 4;
    }
}
#[derive(Debug)]
pub enum AdvSide {
    _Buy,
    _Sell,
    _Trade,
    _Cross,
}
impl Field for AdvTransType {
    fn tag(&self) -> u16 {
        return 5;
    }
}
#[derive(Debug)]
pub enum AdvTransType {
    _Cancel,
    _New,
    _Replace,
}
impl Field for AvgPx {
    fn tag(&self) -> u16 {
        return 6;
    }
}
pub struct AvgPx {
    value: f32,
}
impl Field for BeginSeqNo {
    fn tag(&self) -> u16 {
        return 7;
    }
}
pub struct BeginSeqNo {
    value: u16,
}
impl Field for BeginString {
    fn tag(&self) -> u16 {
        return 8;
    }
}
pub struct BeginString {
    value: char,
}
impl Field for BodyLength {
    fn tag(&self) -> u16 {
        return 9;
    }
}
pub struct BodyLength {
    value: u16,
}
impl Field for CheckSum {
    fn tag(&self) -> u16 {
        return 10;
    }
}
pub struct CheckSum {
    value: char,
}
impl Field for ClOrdID {
    fn tag(&self) -> u16 {
        return 11;
    }
}
pub struct ClOrdID {
    value: char,
}
impl Field for Commission {
    fn tag(&self) -> u16 {
        return 12;
    }
}
pub struct Commission {
    value: f32,
}
impl Field for CommType {
    fn tag(&self) -> u16 {
        return 13;
    }
}
#[derive(Debug)]
pub enum CommType {
    _Pershare,
    _Percentage,
    _Absolute,
}
impl Field for CumQty {
    fn tag(&self) -> u16 {
        return 14;
    }
}
pub struct CumQty {
    value: u16,
}
impl Field for Currency {
    fn tag(&self) -> u16 {
        return 15;
    }
}
pub struct Currency {
    value: char,
}
impl Field for EndSeqNo {
    fn tag(&self) -> u16 {
        return 16;
    }
}
pub struct EndSeqNo {
    value: u16,
}
impl Field for ExecID {
    fn tag(&self) -> u16 {
        return 17;
    }
}
pub struct ExecID {
    value: u16,
}
impl Field for ExecInst {
    fn tag(&self) -> u16 {
        return 18;
    }
}
#[derive(Debug)]
pub enum ExecInst {
    _Stayonofferside,
    _Notheld,
    _Work,
    _Goalong,
    _Overtheday,
    _Held,
    _Participatedontinitiate,
    _Strictscale,
    _Trytoscale,
    _Stayonbidside,
    _Nocross,
    _Oktocross,
    _Callfirst,
    _Percentofvolume,
    _Donotincrease,
    _Donotreduce,
    _Allornone,
    _Institutionsonly,
    _Lastpeg,
    _Midpricepeg,
    _Nonnegotiable,
    _Openingpeg,
    _Marketpeg,
    _Primarypeg,
    _Suspend,
}
impl Field for ExecRefID {
    fn tag(&self) -> u16 {
        return 19;
    }
}
pub struct ExecRefID {
    value: u16,
}
impl Field for ExecTransType {
    fn tag(&self) -> u16 {
        return 20;
    }
}
#[derive(Debug)]
pub enum ExecTransType {
    _New,
    _Cancel,
    _Correct,
    _Status,
}
impl Field for HandlInst {
    fn tag(&self) -> u16 {
        return 21;
    }
}
#[derive(Debug)]
pub enum HandlInst {
    _Automatedexecutionorderprivatenobrokerintervention,
    _Automatedexecutionorderpublicbrokerinterventionok,
    _Manualorderbestexecution,
}
impl Field for IDSource {
    fn tag(&self) -> u16 {
        return 22;
    }
}
#[derive(Debug)]
pub enum IDSource {
    _Cusip,
    _Sedol,
    _Quik,
    _Isinnumber,
    _Riccode,
}
impl Field for IOIid {
    fn tag(&self) -> u16 {
        return 23;
    }
}
pub struct IOIid {
    value: u16,
}
impl Field for IOIOthSvc {
    fn tag(&self) -> u16 {
        return 24;
    }
}
#[derive(Debug)]
pub enum IOIOthSvc {
    _Autex,
    _Bridge,
}
impl Field for IOIQltyInd {
    fn tag(&self) -> u16 {
        return 25;
    }
}
#[derive(Debug)]
pub enum IOIQltyInd {
    _High,
    _Low,
    _Medium,
}
impl Field for IOIRefID {
    fn tag(&self) -> u16 {
        return 26;
    }
}
pub struct IOIRefID {
    value: u16,
}
impl Field for IOIShares {
    fn tag(&self) -> u16 {
        return 27;
    }
}
#[derive(Debug)]
pub enum IOIShares {
    _Large,
    _Medium,
    _Small,
}
impl Field for IOITransType {
    fn tag(&self) -> u16 {
        return 28;
    }
}
#[derive(Debug)]
pub enum IOITransType {
    _Cancel,
    _New,
    _Replace,
}
impl Field for LastCapacity {
    fn tag(&self) -> u16 {
        return 29;
    }
}
#[derive(Debug)]
pub enum LastCapacity {
    _Agent,
    _Crossasagent,
    _Crossasprincipal,
    _Principal,
}
impl Field for LastMkt {
    fn tag(&self) -> u16 {
        return 30;
    }
}
pub struct LastMkt {
    value: char,
}
impl Field for LastPx {
    fn tag(&self) -> u16 {
        return 31;
    }
}
pub struct LastPx {
    value: f32,
}
impl Field for LastShares {
    fn tag(&self) -> u16 {
        return 32;
    }
}
pub struct LastShares {
    value: u16,
}
impl Field for LinesOfText {
    fn tag(&self) -> u16 {
        return 33;
    }
}
pub struct LinesOfText {
    value: u16,
}
impl Field for MsgSeqNum {
    fn tag(&self) -> u16 {
        return 34;
    }
}
pub struct MsgSeqNum {
    value: u16,
}
impl Field for MsgType {
    fn tag(&self) -> u16 {
        return 35;
    }
}
#[derive(Debug)]
pub enum MsgType {
    _Heartbeat,
    _Testrequest,
    _Resendrequest,
    _Reject,
    _Sequencereset,
    _Logout,
    _Indicationofinterest,
    _Advertisement,
    _Executionreport,
    _Ordercancelreject,
    _Logon,
    _News,
    _Email,
    _Orderd,
    _Ordere,
    _Ordercancelrequest,
    _Ordercancelreplacerequest,
    _Orderstatusrequest,
    _Allocation,
    _Listcancelrequest,
    _Listexecute,
    _Liststatusrequest,
    _Liststatus,
    _Allocationack,
    _Dontknowtrade,
    _Quoterequest,
    _Quote,
}
impl Field for NewSeqNo {
    fn tag(&self) -> u16 {
        return 36;
    }
}
pub struct NewSeqNo {
    value: u16,
}
impl Field for OrderID {
    fn tag(&self) -> u16 {
        return 37;
    }
}
pub struct OrderID {
    value: char,
}
impl Field for OrderQty {
    fn tag(&self) -> u16 {
        return 38;
    }
}
pub struct OrderQty {
    value: u16,
}
impl Field for OrdStatus {
    fn tag(&self) -> u16 {
        return 39;
    }
}
#[derive(Debug)]
pub enum OrdStatus {
    _New,
    _Partiallyfilled,
    _Filled,
    _Doneforday,
    _Canceled,
    _Replaced,
    _Pendingcancelreplace,
    _Stopped,
    _Rejected,
    _Suspended,
    _Pendingnew,
    _Calculated,
    _Expired,
}
impl Field for OrdType {
    fn tag(&self) -> u16 {
        return 40;
    }
}
#[derive(Debug)]
pub enum OrdType {
    _Market,
    _Limit,
    _Stop,
    _Stoplimit,
    _Marketonclose,
    _Withorwithout,
    _Limitorbetter,
    _Limitwithorwithout,
    _Onbasis,
    _Onclose,
    _Limitonclose,
    _Forex,
    _Previouslyquoted,
    _Previouslyindicated,
    _Pegged,
}
impl Field for OrigClOrdID {
    fn tag(&self) -> u16 {
        return 41;
    }
}
pub struct OrigClOrdID {
    value: char,
}
impl Field for OrigTime {
    fn tag(&self) -> u16 {
        return 42;
    }
}
pub struct OrigTime {
    value: u64,
}
impl Field for PossDupFlag {
    fn tag(&self) -> u16 {
        return 43;
    }
}
#[derive(Debug)]
pub enum PossDupFlag {
    _No,
    _Yes,
}
impl Field for Price {
    fn tag(&self) -> u16 {
        return 44;
    }
}
pub struct Price {
    value: f32,
}
impl Field for RefSeqNum {
    fn tag(&self) -> u16 {
        return 45;
    }
}
pub struct RefSeqNum {
    value: u16,
}
impl Field for RelatdSym {
    fn tag(&self) -> u16 {
        return 46;
    }
}
pub struct RelatdSym {
    value: char,
}
impl Field for Rule80A {
    fn tag(&self) -> u16 {
        return 47;
    }
}
#[derive(Debug)]
pub enum Rule80A {
    _Agencysingleorder,
    _Programordernonindexarbformemberfirmorg,
    _Programorderindexarbformemberfirmorg,
    _Individualinvestorsingleorder,
    _Programorderindexarbforindividualcustomer,
    _Programordernonindexarbforindividualcustomer,
    _Programorderindexarbforothermember,
    _Programordernonindexarbforothermember,
    _Programorderindexarbforotheragency,
    _Allotherordersasagentforothermember,
    _Programordernonindexarbforotheragency,
}
impl Field for SecurityID {
    fn tag(&self) -> u16 {
        return 48;
    }
}
pub struct SecurityID {
    value: char,
}
impl Field for SenderCompID {
    fn tag(&self) -> u16 {
        return 49;
    }
}
pub struct SenderCompID {
    value: char,
}
impl Field for SenderSubID {
    fn tag(&self) -> u16 {
        return 50;
    }
}
pub struct SenderSubID {
    value: char,
}
impl Field for SendingTime {
    fn tag(&self) -> u16 {
        return 52;
    }
}
pub struct SendingTime {
    value: u64,
}
impl Field for Shares {
    fn tag(&self) -> u16 {
        return 53;
    }
}
pub struct Shares {
    value: u16,
}
impl Field for Side {
    fn tag(&self) -> u16 {
        return 54;
    }
}
#[derive(Debug)]
pub enum Side {
    _Buy,
    _Sell,
    _Buyminus,
    _Sellplus,
    _Sellshort,
    _Sellshortexempt,
}
impl Field for Symbol {
    fn tag(&self) -> u16 {
        return 55;
    }
}
pub struct Symbol {
    value: char,
}
impl Field for TargetCompID {
    fn tag(&self) -> u16 {
        return 56;
    }
}
pub struct TargetCompID {
    value: char,
}
impl Field for TargetSubID {
    fn tag(&self) -> u16 {
        return 57;
    }
}
pub struct TargetSubID {
    value: char,
}
impl Field for Text {
    fn tag(&self) -> u16 {
        return 58;
    }
}
pub struct Text {
    value: char,
}
impl Field for TimeInForce {
    fn tag(&self) -> u16 {
        return 59;
    }
}
#[derive(Debug)]
pub enum TimeInForce {
    _Day,
    _Goodtillcancel,
    _Attheopening,
    _Immediateorcancel,
    _Fillorkill,
    _Goodtillcrossing,
    _Goodtilldate,
}
impl Field for TransactTime {
    fn tag(&self) -> u16 {
        return 60;
    }
}
pub struct TransactTime {
    value: u64,
}
impl Field for Urgency {
    fn tag(&self) -> u16 {
        return 61;
    }
}
#[derive(Debug)]
pub enum Urgency {
    _Normal,
    _Flash,
    _Background,
}
impl Field for ValidUntilTime {
    fn tag(&self) -> u16 {
        return 62;
    }
}
pub struct ValidUntilTime {
    value: u64,
}
impl Field for SettlmntTyp {
    fn tag(&self) -> u16 {
        return 63;
    }
}
#[derive(Debug)]
pub enum SettlmntTyp {
    _Regular,
    _Cash,
    _Nextday,
    _Tplus2,
    _Tplus3,
    _Tplus4,
    _Future,
    _Whenissued,
    _Sellersoption,
    _Tplus5,
}
impl Field for FutSettDate {
    fn tag(&self) -> u16 {
        return 64;
    }
}
pub struct FutSettDate {
    value: String,
}
impl Field for SymbolSfx {
    fn tag(&self) -> u16 {
        return 65;
    }
}
pub struct SymbolSfx {
    value: char,
}
impl Field for ListID {
    fn tag(&self) -> u16 {
        return 66;
    }
}
pub struct ListID {
    value: char,
}
impl Field for ListSeqNo {
    fn tag(&self) -> u16 {
        return 67;
    }
}
pub struct ListSeqNo {
    value: u16,
}
impl Field for ListNoOrds {
    fn tag(&self) -> u16 {
        return 68;
    }
}
pub struct ListNoOrds {
    value: u16,
}
impl Field for ListExecInst {
    fn tag(&self) -> u16 {
        return 69;
    }
}
pub struct ListExecInst {
    value: char,
}
impl Field for AllocID {
    fn tag(&self) -> u16 {
        return 70;
    }
}
pub struct AllocID {
    value: u16,
}
impl Field for AllocTransType {
    fn tag(&self) -> u16 {
        return 71;
    }
}
#[derive(Debug)]
pub enum AllocTransType {
    _New,
    _Replace,
    _Cancel,
}
impl Field for RefAllocID {
    fn tag(&self) -> u16 {
        return 72;
    }
}
pub struct RefAllocID {
    value: u16,
}
impl Field for NoOrders {
    fn tag(&self) -> u16 {
        return 73;
    }
}
pub struct NoOrders {
    value: u16,
}
impl Field for AvgPrxPrecision {
    fn tag(&self) -> u16 {
        return 74;
    }
}
pub struct AvgPrxPrecision {
    value: u16,
}
impl Field for TradeDate {
    fn tag(&self) -> u16 {
        return 75;
    }
}
pub struct TradeDate {
    value: String,
}
impl Field for ExecBroker {
    fn tag(&self) -> u16 {
        return 76;
    }
}
pub struct ExecBroker {
    value: char,
}
impl Field for OpenClose {
    fn tag(&self) -> u16 {
        return 77;
    }
}
pub struct OpenClose {
    value: char,
}
impl Field for NoAllocs {
    fn tag(&self) -> u16 {
        return 78;
    }
}
pub struct NoAllocs {
    value: u16,
}
impl Field for AllocAccount {
    fn tag(&self) -> u16 {
        return 79;
    }
}
pub struct AllocAccount {
    value: char,
}
impl Field for AllocShares {
    fn tag(&self) -> u16 {
        return 80;
    }
}
pub struct AllocShares {
    value: u16,
}
impl Field for ProcessCode {
    fn tag(&self) -> u16 {
        return 81;
    }
}
#[derive(Debug)]
pub enum ProcessCode {
    _Regular,
    _Softdollar,
    _Stepin,
    _Stepout,
    _Softdollarstepin,
    _Softdollarstepout,
    _Plansponsor,
}
impl Field for NoRpts {
    fn tag(&self) -> u16 {
        return 82;
    }
}
pub struct NoRpts {
    value: u16,
}
impl Field for RptSeq {
    fn tag(&self) -> u16 {
        return 83;
    }
}
pub struct RptSeq {
    value: u16,
}
impl Field for CxlQty {
    fn tag(&self) -> u16 {
        return 84;
    }
}
pub struct CxlQty {
    value: u16,
}
impl Field for NoDlvyInst {
    fn tag(&self) -> u16 {
        return 85;
    }
}
pub struct NoDlvyInst {
    value: u16,
}
impl Field for DlvyInst {
    fn tag(&self) -> u16 {
        return 86;
    }
}
pub struct DlvyInst {
    value: char,
}
impl Field for AllocStatus {
    fn tag(&self) -> u16 {
        return 87;
    }
}
#[derive(Debug)]
pub enum AllocStatus {
    _Accepted,
    _Rejected,
    _Partialaccept,
    _Received,
}
impl Field for AllocRejCode {
    fn tag(&self) -> u16 {
        return 88;
    }
}
#[derive(Debug)]
pub enum AllocRejCode {
    _Unknownaccount,
    _Incorrectquantity,
    _Incorrectaverageprice,
    _Unknownexecutingbrokermnemonic,
    _Commissiondifference,
    _Unknownorderid,
    _Unknownlistid,
    _Other,
}
impl Field for Signature {
    fn tag(&self) -> u16 {
        return 89;
    }
}
pub struct Signature {
    value: [u8; 1024],
}
impl Field for SecureDataLen {
    fn tag(&self) -> u16 {
        return 90;
    }
}
pub struct SecureDataLen {
    value: usize,
}
impl Field for SecureData {
    fn tag(&self) -> u16 {
        return 91;
    }
}
pub struct SecureData {
    value: [u8; 1024],
}
impl Field for BrokerOfCredit {
    fn tag(&self) -> u16 {
        return 92;
    }
}
pub struct BrokerOfCredit {
    value: char,
}
impl Field for SignatureLength {
    fn tag(&self) -> u16 {
        return 93;
    }
}
pub struct SignatureLength {
    value: usize,
}
impl Field for EmailType {
    fn tag(&self) -> u16 {
        return 94;
    }
}
#[derive(Debug)]
pub enum EmailType {
    _New,
    _Reply,
    _Adminreply,
}
impl Field for RawDataLength {
    fn tag(&self) -> u16 {
        return 95;
    }
}
pub struct RawDataLength {
    value: usize,
}
impl Field for RawData {
    fn tag(&self) -> u16 {
        return 96;
    }
}
pub struct RawData {
    value: [u8; 1024],
}
impl Field for PossResend {
    fn tag(&self) -> u16 {
        return 97;
    }
}
pub struct PossResend {
    value: char,
}
impl Field for EncryptMethod {
    fn tag(&self) -> u16 {
        return 98;
    }
}
#[derive(Debug)]
pub enum EncryptMethod {
    _None,
    _Pkcs,
    _Des,
    _Pkcsdes,
    _Pgpdes,
    _Pgpdesmd5,
    _Pemdesmd5,
}
impl Field for StopPx {
    fn tag(&self) -> u16 {
        return 99;
    }
}
pub struct StopPx {
    value: f32,
}
impl Field for ExDestination {
    fn tag(&self) -> u16 {
        return 100;
    }
}
#[derive(Debug)]
pub enum ExDestination {
    _None,
    _Posit,
}
impl Field for CxlRejReason {
    fn tag(&self) -> u16 {
        return 102;
    }
}
#[derive(Debug)]
pub enum CxlRejReason {
    _Toolatetocancel,
    _Unknownorder,
}
impl Field for OrdRejReason {
    fn tag(&self) -> u16 {
        return 103;
    }
}
#[derive(Debug)]
pub enum OrdRejReason {
    _Brokeroption,
    _Unknownsymbol,
    _Exchangeclosed,
    _Orderexceedslimit,
    _Toolatetoenter,
}
impl Field for IOIQualifier {
    fn tag(&self) -> u16 {
        return 104;
    }
}
#[derive(Debug)]
pub enum IOIQualifier {
    _Allornone,
    _Attheclose,
    _Intouchwith,
    _Limit,
    _Morebehind,
    _Attheopen,
    _Takingaposition,
    _Currentquote,
    _Portfolioshown,
    _Throughtheday,
    _Versus,
    _Indication,
    _Crossingopportunity,
}
impl Field for WaveNo {
    fn tag(&self) -> u16 {
        return 105;
    }
}
pub struct WaveNo {
    value: char,
}
impl Field for Issuer {
    fn tag(&self) -> u16 {
        return 106;
    }
}
pub struct Issuer {
    value: char,
}
impl Field for SecurityDesc {
    fn tag(&self) -> u16 {
        return 107;
    }
}
pub struct SecurityDesc {
    value: char,
}
impl Field for HeartBtInt {
    fn tag(&self) -> u16 {
        return 108;
    }
}
pub struct HeartBtInt {
    value: u16,
}
impl Field for ClientID {
    fn tag(&self) -> u16 {
        return 109;
    }
}
pub struct ClientID {
    value: char,
}
impl Field for MinQty {
    fn tag(&self) -> u16 {
        return 110;
    }
}
pub struct MinQty {
    value: u16,
}
impl Field for MaxFloor {
    fn tag(&self) -> u16 {
        return 111;
    }
}
pub struct MaxFloor {
    value: u16,
}
impl Field for TestReqID {
    fn tag(&self) -> u16 {
        return 112;
    }
}
pub struct TestReqID {
    value: char,
}
impl Field for ReportToExch {
    fn tag(&self) -> u16 {
        return 113;
    }
}
#[derive(Debug)]
pub enum ReportToExch {
    _No,
    _Yes,
}
impl Field for LocateReqd {
    fn tag(&self) -> u16 {
        return 114;
    }
}
#[derive(Debug)]
pub enum LocateReqd {
    _No,
    _Yes,
}
impl Field for OnBehalfOfCompID {
    fn tag(&self) -> u16 {
        return 115;
    }
}
pub struct OnBehalfOfCompID {
    value: char,
}
impl Field for OnBehalfOfSubID {
    fn tag(&self) -> u16 {
        return 116;
    }
}
pub struct OnBehalfOfSubID {
    value: char,
}
impl Field for QuoteID {
    fn tag(&self) -> u16 {
        return 117;
    }
}
pub struct QuoteID {
    value: char,
}
impl Field for NetMoney {
    fn tag(&self) -> u16 {
        return 118;
    }
}
pub struct NetMoney {
    value: f32,
}
impl Field for SettlCurrAmt {
    fn tag(&self) -> u16 {
        return 119;
    }
}
pub struct SettlCurrAmt {
    value: f32,
}
impl Field for SettlCurrency {
    fn tag(&self) -> u16 {
        return 120;
    }
}
pub struct SettlCurrency {
    value: char,
}
impl Field for ForexReq {
    fn tag(&self) -> u16 {
        return 121;
    }
}
#[derive(Debug)]
pub enum ForexReq {
    _No,
    _Yes,
}
impl Field for OrigSendingTime {
    fn tag(&self) -> u16 {
        return 122;
    }
}
pub struct OrigSendingTime {
    value: u64,
}
impl Field for GapFillFlag {
    fn tag(&self) -> u16 {
        return 123;
    }
}
#[derive(Debug)]
pub enum GapFillFlag {
    _No,
    _Yes,
}
impl Field for NoExecs {
    fn tag(&self) -> u16 {
        return 124;
    }
}
pub struct NoExecs {
    value: u16,
}
impl Field for CxlType {
    fn tag(&self) -> u16 {
        return 125;
    }
}
#[derive(Debug)]
pub enum CxlType {
    _Fullremainingquantity,
    _Partialcancel,
}
impl Field for ExpireTime {
    fn tag(&self) -> u16 {
        return 126;
    }
}
pub struct ExpireTime {
    value: u64,
}
impl Field for DKReason {
    fn tag(&self) -> u16 {
        return 127;
    }
}
#[derive(Debug)]
pub enum DKReason {
    _Unknownsymbol,
    _Wrongside,
    _Quantityexceedsorder,
    _Nomatchingorder,
    _Priceexceedslimit,
    _Other,
}
impl Field for DeliverToCompID {
    fn tag(&self) -> u16 {
        return 128;
    }
}
pub struct DeliverToCompID {
    value: char,
}
impl Field for DeliverToSubID {
    fn tag(&self) -> u16 {
        return 129;
    }
}
pub struct DeliverToSubID {
    value: char,
}
impl Field for IOINaturalFlag {
    fn tag(&self) -> u16 {
        return 130;
    }
}
#[derive(Debug)]
pub enum IOINaturalFlag {
    _No,
    _Yes,
}
impl Field for QuoteReqID {
    fn tag(&self) -> u16 {
        return 131;
    }
}
pub struct QuoteReqID {
    value: char,
}
impl Field for BidPx {
    fn tag(&self) -> u16 {
        return 132;
    }
}
pub struct BidPx {
    value: f32,
}
impl Field for OfferPx {
    fn tag(&self) -> u16 {
        return 133;
    }
}
pub struct OfferPx {
    value: f32,
}
impl Field for BidSize {
    fn tag(&self) -> u16 {
        return 134;
    }
}
pub struct BidSize {
    value: u16,
}
impl Field for OfferSize {
    fn tag(&self) -> u16 {
        return 135;
    }
}
pub struct OfferSize {
    value: u16,
}
impl Field for NoMiscFees {
    fn tag(&self) -> u16 {
        return 136;
    }
}
pub struct NoMiscFees {
    value: u16,
}
impl Field for MiscFeeAmt {
    fn tag(&self) -> u16 {
        return 137;
    }
}
pub struct MiscFeeAmt {
    value: f32,
}
impl Field for MiscFeeCurr {
    fn tag(&self) -> u16 {
        return 138;
    }
}
pub struct MiscFeeCurr {
    value: char,
}
impl Field for MiscFeeType {
    fn tag(&self) -> u16 {
        return 139;
    }
}
#[derive(Debug)]
pub enum MiscFeeType {
    _Regulatory,
    _Tax,
    _Localcommission,
    _Exchangefees,
    _Stamp,
    _Levy,
    _Other,
}
impl Field for PrevClosePx {
    fn tag(&self) -> u16 {
        return 140;
    }
}
pub struct PrevClosePx {
    value: f32,
}
