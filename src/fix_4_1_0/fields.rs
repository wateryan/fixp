use field::Field;

impl Field for Account {
    fn tag(&self) -> u8 {
        return 1;
    }
}
pub struct Account {
    value: char,
}
impl Field for AdvId {
    fn tag(&self) -> u8 {
        return 2;
    }
}
pub struct AdvId {
    value: char,
}
impl Field for AdvRefID {
    fn tag(&self) -> u8 {
        return 3;
    }
}
pub struct AdvRefID {
    value: char,
}
impl Field for AdvSide {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 6;
    }
}
pub struct AvgPx {
    value: f32,
}
impl Field for BeginSeqNo {
    fn tag(&self) -> u8 {
        return 7;
    }
}
pub struct BeginSeqNo {
    value: u16,
}
impl Field for BeginString {
    fn tag(&self) -> u8 {
        return 8;
    }
}
pub struct BeginString {
    value: char,
}
impl Field for BodyLength {
    fn tag(&self) -> u8 {
        return 9;
    }
}
pub struct BodyLength {
    value: u16,
}
impl Field for CheckSum {
    fn tag(&self) -> u8 {
        return 10;
    }
}
pub struct CheckSum {
    value: char,
}
impl Field for ClOrdID {
    fn tag(&self) -> u8 {
        return 11;
    }
}
pub struct ClOrdID {
    value: char,
}
impl Field for Commission {
    fn tag(&self) -> u8 {
        return 12;
    }
}
pub struct Commission {
    value: f32,
}
impl Field for CommType {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 14;
    }
}
pub struct CumQty {
    value: u16,
}
impl Field for Currency {
    fn tag(&self) -> u8 {
        return 15;
    }
}
pub struct Currency {
    value: char,
}
impl Field for EndSeqNo {
    fn tag(&self) -> u8 {
        return 16;
    }
}
pub struct EndSeqNo {
    value: u16,
}
impl Field for ExecID {
    fn tag(&self) -> u8 {
        return 17;
    }
}
pub struct ExecID {
    value: char,
}
impl Field for ExecInst {
    fn tag(&self) -> u8 {
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
    _Customerdisplayinstruction,
    _Netting,
}
impl Field for ExecRefID {
    fn tag(&self) -> u8 {
        return 19;
    }
}
pub struct ExecRefID {
    value: char,
}
impl Field for ExecTransType {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
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
    _Isocurrencycode,
    _Isocountrycode,
}
impl Field for IOIid {
    fn tag(&self) -> u8 {
        return 23;
    }
}
pub struct IOIid {
    value: char,
}
impl Field for IOIOthSvc {
    fn tag(&self) -> u8 {
        return 24;
    }
}
#[derive(Debug)]
pub enum IOIOthSvc {
    _Autex,
    _Bridge,
}
impl Field for IOIQltyInd {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 26;
    }
}
pub struct IOIRefID {
    value: char,
}
impl Field for IOIShares {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 30;
    }
}
pub struct LastMkt {
    value: char,
}
impl Field for LastPx {
    fn tag(&self) -> u8 {
        return 31;
    }
}
pub struct LastPx {
    value: f32,
}
impl Field for LastShares {
    fn tag(&self) -> u8 {
        return 32;
    }
}
pub struct LastShares {
    value: u16,
}
impl Field for LinesOfText {
    fn tag(&self) -> u8 {
        return 33;
    }
}
pub struct LinesOfText {
    value: u16,
}
impl Field for MsgSeqNum {
    fn tag(&self) -> u8 {
        return 34;
    }
}
pub struct MsgSeqNum {
    value: u16,
}
impl Field for MsgType {
    fn tag(&self) -> u8 {
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
    _Settlementinstructions,
}
impl Field for NewSeqNo {
    fn tag(&self) -> u8 {
        return 36;
    }
}
pub struct NewSeqNo {
    value: u16,
}
impl Field for OrderID {
    fn tag(&self) -> u8 {
        return 37;
    }
}
pub struct OrderID {
    value: char,
}
impl Field for OrderQty {
    fn tag(&self) -> u8 {
        return 38;
    }
}
pub struct OrderQty {
    value: u16,
}
impl Field for OrdStatus {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
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
    _Forexc,
    _Previouslyquoted,
    _Previouslyindicated,
    _Forexf,
    _Forexg,
    _Forexh,
    _Pegged,
}
impl Field for OrigClOrdID {
    fn tag(&self) -> u8 {
        return 41;
    }
}
pub struct OrigClOrdID {
    value: char,
}
impl Field for OrigTime {
    fn tag(&self) -> u8 {
        return 42;
    }
}
pub struct OrigTime {
    value: u64,
}
impl Field for PossDupFlag {
    fn tag(&self) -> u8 {
        return 43;
    }
}
#[derive(Debug)]
pub enum PossDupFlag {
    _No,
    _Yes,
}
impl Field for Price {
    fn tag(&self) -> u8 {
        return 44;
    }
}
pub struct Price {
    value: f32,
}
impl Field for RefSeqNum {
    fn tag(&self) -> u8 {
        return 45;
    }
}
pub struct RefSeqNum {
    value: u16,
}
impl Field for RelatdSym {
    fn tag(&self) -> u8 {
        return 46;
    }
}
pub struct RelatdSym {
    value: char,
}
impl Field for Rule80A {
    fn tag(&self) -> u8 {
        return 47;
    }
}
#[derive(Debug)]
pub enum Rule80A {
    _Agencysingleorder,
    _Shortexempttransactionb,
    _Programordernonindexarbformemberfirmorg,
    _Programorderindexarbformemberfirmorg,
    _Registeredequitymarketmakertrades,
    _Shortexempttransactionf,
    _Shortexempttransactionh,
    _Individualinvestorsingleorder,
    _Programorderindexarbforindividualcustomer,
    _Programordernonindexarbforindividualcustomer,
    _Shortexempttransactionformembercompetingmarketmakeraffiliatedwiththefirmclearingthetrade,
    _Programorderindexarbforothermember,
    _Programordernonindexarbforothermember,
    _Competingdealertradeso,
    _Principal,
    _Competingdealertradesr,
    _Specialisttrades,
    _Competingdealertradest,
    _Programorderindexarbforotheragency,
    _Allotherordersasagentforothermember,
    _Shortexempttransactionformembercompetingmarketmakernotaffiliatedwiththefirmclearingthetrade,
    _Programordernonindexarbforotheragency,
    _Shortexempttransactionfornonmembercompetingmarketmaker,
}
impl Field for SecurityID {
    fn tag(&self) -> u8 {
        return 48;
    }
}
pub struct SecurityID {
    value: char,
}
impl Field for SenderCompID {
    fn tag(&self) -> u8 {
        return 49;
    }
}
pub struct SenderCompID {
    value: char,
}
impl Field for SenderSubID {
    fn tag(&self) -> u8 {
        return 50;
    }
}
pub struct SenderSubID {
    value: char,
}
impl Field for SendingDate {
    fn tag(&self) -> u8 {
        return 51;
    }
}
pub struct SendingDate {
    value: String,
}
impl Field for SendingTime {
    fn tag(&self) -> u8 {
        return 52;
    }
}
pub struct SendingTime {
    value: u64,
}
impl Field for Shares {
    fn tag(&self) -> u8 {
        return 53;
    }
}
pub struct Shares {
    value: u16,
}
impl Field for Side {
    fn tag(&self) -> u8 {
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
    _Undisclosed,
    _Cross,
}
impl Field for Symbol {
    fn tag(&self) -> u8 {
        return 55;
    }
}
pub struct Symbol {
    value: char,
}
impl Field for TargetCompID {
    fn tag(&self) -> u8 {
        return 56;
    }
}
pub struct TargetCompID {
    value: char,
}
impl Field for TargetSubID {
    fn tag(&self) -> u8 {
        return 57;
    }
}
pub struct TargetSubID {
    value: char,
}
impl Field for Text {
    fn tag(&self) -> u8 {
        return 58;
    }
}
pub struct Text {
    value: char,
}
impl Field for TimeInForce {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 60;
    }
}
pub struct TransactTime {
    value: u64,
}
impl Field for Urgency {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 62;
    }
}
pub struct ValidUntilTime {
    value: u64,
}
impl Field for SettlmntTyp {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 64;
    }
}
pub struct FutSettDate {
    value: String,
}
impl Field for SymbolSfx {
    fn tag(&self) -> u8 {
        return 65;
    }
}
pub struct SymbolSfx {
    value: char,
}
impl Field for ListID {
    fn tag(&self) -> u8 {
        return 66;
    }
}
pub struct ListID {
    value: char,
}
impl Field for ListSeqNo {
    fn tag(&self) -> u8 {
        return 67;
    }
}
pub struct ListSeqNo {
    value: u16,
}
impl Field for ListNoOrds {
    fn tag(&self) -> u8 {
        return 68;
    }
}
pub struct ListNoOrds {
    value: u16,
}
impl Field for ListExecInst {
    fn tag(&self) -> u8 {
        return 69;
    }
}
pub struct ListExecInst {
    value: char,
}
impl Field for AllocID {
    fn tag(&self) -> u8 {
        return 70;
    }
}
pub struct AllocID {
    value: char,
}
impl Field for AllocTransType {
    fn tag(&self) -> u8 {
        return 71;
    }
}
#[derive(Debug)]
pub enum AllocTransType {
    _New,
    _Replace,
    _Cancel,
    _Preliminary,
    _Calculated,
}
impl Field for RefAllocID {
    fn tag(&self) -> u8 {
        return 72;
    }
}
pub struct RefAllocID {
    value: char,
}
impl Field for NoOrders {
    fn tag(&self) -> u8 {
        return 73;
    }
}
pub struct NoOrders {
    value: u16,
}
impl Field for AvgPrxPrecision {
    fn tag(&self) -> u8 {
        return 74;
    }
}
pub struct AvgPrxPrecision {
    value: u16,
}
impl Field for TradeDate {
    fn tag(&self) -> u8 {
        return 75;
    }
}
pub struct TradeDate {
    value: String,
}
impl Field for ExecBroker {
    fn tag(&self) -> u8 {
        return 76;
    }
}
pub struct ExecBroker {
    value: char,
}
impl Field for OpenClose {
    fn tag(&self) -> u8 {
        return 77;
    }
}
#[derive(Debug)]
pub enum OpenClose {
    _Close,
    _Open,
}
impl Field for NoAllocs {
    fn tag(&self) -> u8 {
        return 78;
    }
}
pub struct NoAllocs {
    value: u16,
}
impl Field for AllocAccount {
    fn tag(&self) -> u8 {
        return 79;
    }
}
pub struct AllocAccount {
    value: char,
}
impl Field for AllocShares {
    fn tag(&self) -> u8 {
        return 80;
    }
}
pub struct AllocShares {
    value: u16,
}
impl Field for ProcessCode {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 82;
    }
}
pub struct NoRpts {
    value: u16,
}
impl Field for RptSeq {
    fn tag(&self) -> u8 {
        return 83;
    }
}
pub struct RptSeq {
    value: u16,
}
impl Field for CxlQty {
    fn tag(&self) -> u8 {
        return 84;
    }
}
pub struct CxlQty {
    value: u16,
}
impl Field for NoDlvyInst {
    fn tag(&self) -> u8 {
        return 85;
    }
}
pub struct NoDlvyInst {
    value: u16,
}
impl Field for DlvyInst {
    fn tag(&self) -> u8 {
        return 86;
    }
}
pub struct DlvyInst {
    value: char,
}
impl Field for AllocStatus {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 89;
    }
}
pub struct Signature {
    value: [u8; 1024],
}
impl Field for SecureDataLen {
    fn tag(&self) -> u8 {
        return 90;
    }
}
pub struct SecureDataLen {
    value: usize,
}
impl Field for SecureData {
    fn tag(&self) -> u8 {
        return 91;
    }
}
pub struct SecureData {
    value: [u8; 1024],
}
impl Field for BrokerOfCredit {
    fn tag(&self) -> u8 {
        return 92;
    }
}
pub struct BrokerOfCredit {
    value: char,
}
impl Field for SignatureLength {
    fn tag(&self) -> u8 {
        return 93;
    }
}
pub struct SignatureLength {
    value: usize,
}
impl Field for EmailType {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 95;
    }
}
pub struct RawDataLength {
    value: usize,
}
impl Field for RawData {
    fn tag(&self) -> u8 {
        return 96;
    }
}
pub struct RawData {
    value: [u8; 1024],
}
impl Field for PossResend {
    fn tag(&self) -> u8 {
        return 97;
    }
}
#[derive(Debug)]
pub enum PossResend {
    _No,
    _Yes,
}
impl Field for EncryptMethod {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 99;
    }
}
pub struct StopPx {
    value: f32,
}
impl Field for ExDestination {
    fn tag(&self) -> u8 {
        return 100;
    }
}
pub struct ExDestination {
    value: char,
}
impl Field for CxlRejReason {
    fn tag(&self) -> u8 {
        return 102;
    }
}
#[derive(Debug)]
pub enum CxlRejReason {
    _Toolatetocancel,
    _Unknownorder,
}
impl Field for OrdRejReason {
    fn tag(&self) -> u8 {
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
    _Unknownorder,
    _Duplicateorder,
}
impl Field for IOIQualifier {
    fn tag(&self) -> u8 {
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
    _Atthemarket,
    _Portfolioshown,
    _Throughtheday,
    _Versus,
    _Indication,
    _Crossingopportunity,
    _Atthemidpoint,
    _Preopen,
}
impl Field for WaveNo {
    fn tag(&self) -> u8 {
        return 105;
    }
}
pub struct WaveNo {
    value: char,
}
impl Field for Issuer {
    fn tag(&self) -> u8 {
        return 106;
    }
}
pub struct Issuer {
    value: char,
}
impl Field for SecurityDesc {
    fn tag(&self) -> u8 {
        return 107;
    }
}
pub struct SecurityDesc {
    value: char,
}
impl Field for HeartBtInt {
    fn tag(&self) -> u8 {
        return 108;
    }
}
pub struct HeartBtInt {
    value: u16,
}
impl Field for ClientID {
    fn tag(&self) -> u8 {
        return 109;
    }
}
pub struct ClientID {
    value: char,
}
impl Field for MinQty {
    fn tag(&self) -> u8 {
        return 110;
    }
}
pub struct MinQty {
    value: u16,
}
impl Field for MaxFloor {
    fn tag(&self) -> u8 {
        return 111;
    }
}
pub struct MaxFloor {
    value: u16,
}
impl Field for TestReqID {
    fn tag(&self) -> u8 {
        return 112;
    }
}
pub struct TestReqID {
    value: char,
}
impl Field for ReportToExch {
    fn tag(&self) -> u8 {
        return 113;
    }
}
#[derive(Debug)]
pub enum ReportToExch {
    _No,
    _Yes,
}
impl Field for LocateReqd {
    fn tag(&self) -> u8 {
        return 114;
    }
}
#[derive(Debug)]
pub enum LocateReqd {
    _No,
    _Yes,
}
impl Field for OnBehalfOfCompID {
    fn tag(&self) -> u8 {
        return 115;
    }
}
pub struct OnBehalfOfCompID {
    value: char,
}
impl Field for OnBehalfOfSubID {
    fn tag(&self) -> u8 {
        return 116;
    }
}
pub struct OnBehalfOfSubID {
    value: char,
}
impl Field for QuoteID {
    fn tag(&self) -> u8 {
        return 117;
    }
}
pub struct QuoteID {
    value: char,
}
impl Field for NetMoney {
    fn tag(&self) -> u8 {
        return 118;
    }
}
pub struct NetMoney {
    value: f32,
}
impl Field for SettlCurrAmt {
    fn tag(&self) -> u8 {
        return 119;
    }
}
pub struct SettlCurrAmt {
    value: f32,
}
impl Field for SettlCurrency {
    fn tag(&self) -> u8 {
        return 120;
    }
}
pub struct SettlCurrency {
    value: char,
}
impl Field for ForexReq {
    fn tag(&self) -> u8 {
        return 121;
    }
}
#[derive(Debug)]
pub enum ForexReq {
    _No,
    _Yes,
}
impl Field for OrigSendingTime {
    fn tag(&self) -> u8 {
        return 122;
    }
}
pub struct OrigSendingTime {
    value: u64,
}
impl Field for GapFillFlag {
    fn tag(&self) -> u8 {
        return 123;
    }
}
#[derive(Debug)]
pub enum GapFillFlag {
    _No,
    _Yes,
}
impl Field for NoExecs {
    fn tag(&self) -> u8 {
        return 124;
    }
}
pub struct NoExecs {
    value: u16,
}
impl Field for CxlType {
    fn tag(&self) -> u8 {
        return 125;
    }
}
pub struct CxlType {
    value: char,
}
impl Field for ExpireTime {
    fn tag(&self) -> u8 {
        return 126;
    }
}
pub struct ExpireTime {
    value: u64,
}
impl Field for DKReason {
    fn tag(&self) -> u8 {
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
    fn tag(&self) -> u8 {
        return 128;
    }
}
pub struct DeliverToCompID {
    value: char,
}
impl Field for DeliverToSubID {
    fn tag(&self) -> u8 {
        return 129;
    }
}
pub struct DeliverToSubID {
    value: char,
}
impl Field for IOINaturalFlag {
    fn tag(&self) -> u8 {
        return 130;
    }
}
#[derive(Debug)]
pub enum IOINaturalFlag {
    _No,
    _Yes,
}
impl Field for QuoteReqID {
    fn tag(&self) -> u8 {
        return 131;
    }
}
pub struct QuoteReqID {
    value: char,
}
impl Field for BidPx {
    fn tag(&self) -> u8 {
        return 132;
    }
}
pub struct BidPx {
    value: f32,
}
impl Field for OfferPx {
    fn tag(&self) -> u8 {
        return 133;
    }
}
pub struct OfferPx {
    value: f32,
}
impl Field for BidSize {
    fn tag(&self) -> u8 {
        return 134;
    }
}
pub struct BidSize {
    value: u16,
}
impl Field for OfferSize {
    fn tag(&self) -> u8 {
        return 135;
    }
}
pub struct OfferSize {
    value: u16,
}
impl Field for NoMiscFees {
    fn tag(&self) -> u8 {
        return 136;
    }
}
pub struct NoMiscFees {
    value: u16,
}
impl Field for MiscFeeAmt {
    fn tag(&self) -> u8 {
        return 137;
    }
}
pub struct MiscFeeAmt {
    value: f32,
}
impl Field for MiscFeeCurr {
    fn tag(&self) -> u8 {
        return 138;
    }
}
pub struct MiscFeeCurr {
    value: char,
}
impl Field for MiscFeeType {
    fn tag(&self) -> u8 {
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
    _Markup,
}
impl Field for PrevClosePx {
    fn tag(&self) -> u8 {
        return 140;
    }
}
pub struct PrevClosePx {
    value: f32,
}
impl Field for ResetSeqNumFlag {
    fn tag(&self) -> u8 {
        return 141;
    }
}
#[derive(Debug)]
pub enum ResetSeqNumFlag {
    _No,
    _Yes,
}
impl Field for SenderLocationID {
    fn tag(&self) -> u8 {
        return 142;
    }
}
pub struct SenderLocationID {
    value: char,
}
impl Field for TargetLocationID {
    fn tag(&self) -> u8 {
        return 143;
    }
}
pub struct TargetLocationID {
    value: char,
}
impl Field for OnBehalfOfLocationID {
    fn tag(&self) -> u8 {
        return 144;
    }
}
pub struct OnBehalfOfLocationID {
    value: char,
}
impl Field for DeliverToLocationID {
    fn tag(&self) -> u8 {
        return 145;
    }
}
pub struct DeliverToLocationID {
    value: char,
}
impl Field for NoRelatedSym {
    fn tag(&self) -> u8 {
        return 146;
    }
}
pub struct NoRelatedSym {
    value: u16,
}
impl Field for Subject {
    fn tag(&self) -> u8 {
        return 147;
    }
}
pub struct Subject {
    value: char,
}
impl Field for Headline {
    fn tag(&self) -> u8 {
        return 148;
    }
}
pub struct Headline {
    value: char,
}
impl Field for URLLink {
    fn tag(&self) -> u8 {
        return 149;
    }
}
pub struct URLLink {
    value: char,
}
impl Field for ExecType {
    fn tag(&self) -> u8 {
        return 150;
    }
}
#[derive(Debug)]
pub enum ExecType {
    _New,
    _Partialfill,
    _Fill,
    _Doneforday,
    _Cancelled,
    _Replace,
    _Pendingcancelreplace,
    _Stopped,
    _Rejected,
    _Suspended,
    _Pendingnew,
    _Calculated,
    _Expired,
}
impl Field for LeavesQty {
    fn tag(&self) -> u8 {
        return 151;
    }
}
pub struct LeavesQty {
    value: u16,
}
impl Field for CashOrderQty {
    fn tag(&self) -> u8 {
        return 152;
    }
}
pub struct CashOrderQty {
    value: f32,
}
impl Field for AllocAvgPx {
    fn tag(&self) -> u8 {
        return 153;
    }
}
pub struct AllocAvgPx {
    value: f32,
}
impl Field for AllocNetMoney {
    fn tag(&self) -> u8 {
        return 154;
    }
}
pub struct AllocNetMoney {
    value: f32,
}
impl Field for SettlCurrFxRate {
    fn tag(&self) -> u8 {
        return 155;
    }
}
pub struct SettlCurrFxRate {
    value: f32,
}
impl Field for SettlCurrFxRateCalc {
    fn tag(&self) -> u8 {
        return 156;
    }
}
#[derive(Debug)]
pub enum SettlCurrFxRateCalc {
    _Multiply,
    _Divide,
}
impl Field for NumDaysInterest {
    fn tag(&self) -> u8 {
        return 157;
    }
}
pub struct NumDaysInterest {
    value: u16,
}
impl Field for AccruedInterestRate {
    fn tag(&self) -> u8 {
        return 158;
    }
}
pub struct AccruedInterestRate {
    value: f32,
}
impl Field for AccruedInterestAmt {
    fn tag(&self) -> u8 {
        return 159;
    }
}
pub struct AccruedInterestAmt {
    value: f32,
}
impl Field for SettlInstMode {
    fn tag(&self) -> u8 {
        return 160;
    }
}
#[derive(Debug)]
pub enum SettlInstMode {
    _Default,
    _Standinginstructionsprovided,
    _Specificallocationaccountoverriding,
    _Specificallocationaccountstanding,
}
impl Field for AllocText {
    fn tag(&self) -> u8 {
        return 161;
    }
}
pub struct AllocText {
    value: char,
}
impl Field for SettlInstID {
    fn tag(&self) -> u8 {
        return 162;
    }
}
pub struct SettlInstID {
    value: char,
}
impl Field for SettlInstTransType {
    fn tag(&self) -> u8 {
        return 163;
    }
}
#[derive(Debug)]
pub enum SettlInstTransType {
    _Cancel,
    _New,
    _Replace,
}
impl Field for EmailThreadID {
    fn tag(&self) -> u8 {
        return 164;
    }
}
pub struct EmailThreadID {
    value: char,
}
impl Field for SettlInstSource {
    fn tag(&self) -> u8 {
        return 165;
    }
}
#[derive(Debug)]
pub enum SettlInstSource {
    _Brokersinstructions,
    _Institutionsinstructions,
}
impl Field for SettlLocation {
    fn tag(&self) -> u8 {
        return 166;
    }
}
#[derive(Debug)]
pub enum SettlLocation {
    _Cedel,
    _Depositorytrustcompany,
    _Euroclear,
    _Federalbookentry,
    _Localmarketsettlelocation,
    _Physical,
    _Participanttrustcompany,
}
impl Field for SecurityType {
    fn tag(&self) -> u8 {
        return 167;
    }
}
#[derive(Debug)]
pub enum SecurityType {
    _Bankersacceptance,
    _Certificateofdeposit,
    _Collateralizemortgageobligation,
    _Corporatebond,
    _Commercialpaper,
    _Corporateprivateplacement,
    _Commonstock,
    _Federalhousingauthority,
    _Federalhomeloan,
    _Federalnationalmortgageassociation,
    _Foreignexchangecontract,
    _Future,
    _Governmentnationalmortgageassociation,
    _Treasuriesplusagencydebenture,
    _Mutualfund,
    _Mortgageinterestonly,
    _Mortgageprincipleonly,
    _Mortgageprivateplacement,
    _Miscellaneouspassthru,
    _Municipalbond,
    _Noisitcsecuritytype,
    _Option,
    _Preferredstock,
    _Repurchaseagreement,
    _Reverserepurchaseagreement,
    _Studentloanmarketingassociation,
    _Timedeposit,
    _Ustreasurybill,
    _Warrant,
    _Catstigerslions,
}
impl Field for EffectiveTime {
    fn tag(&self) -> u8 {
        return 168;
    }
}
pub struct EffectiveTime {
    value: u64,
}
impl Field for StandInstDbType {
    fn tag(&self) -> u8 {
        return 169;
    }
}
#[derive(Debug)]
pub enum StandInstDbType {
    _Other,
    _Dtcsid,
    _Thomsonalert,
    _Aglobalcustodian,
}
impl Field for StandInstDbName {
    fn tag(&self) -> u8 {
        return 170;
    }
}
pub struct StandInstDbName {
    value: char,
}
impl Field for StandInstDbID {
    fn tag(&self) -> u8 {
        return 171;
    }
}
pub struct StandInstDbID {
    value: char,
}
impl Field for SettlDeliveryType {
    fn tag(&self) -> u8 {
        return 172;
    }
}
pub struct SettlDeliveryType {
    value: u16,
}
impl Field for SettlDepositoryCode {
    fn tag(&self) -> u8 {
        return 173;
    }
}
pub struct SettlDepositoryCode {
    value: char,
}
impl Field for SettlBrkrCode {
    fn tag(&self) -> u8 {
        return 174;
    }
}
pub struct SettlBrkrCode {
    value: char,
}
impl Field for SettlInstCode {
    fn tag(&self) -> u8 {
        return 175;
    }
}
pub struct SettlInstCode {
    value: char,
}
impl Field for SecuritySettlAgentName {
    fn tag(&self) -> u8 {
        return 176;
    }
}
pub struct SecuritySettlAgentName {
    value: char,
}
impl Field for SecuritySettlAgentCode {
    fn tag(&self) -> u8 {
        return 177;
    }
}
pub struct SecuritySettlAgentCode {
    value: char,
}
impl Field for SecuritySettlAgentAcctNum {
    fn tag(&self) -> u8 {
        return 178;
    }
}
pub struct SecuritySettlAgentAcctNum {
    value: char,
}
impl Field for SecuritySettlAgentAcctName {
    fn tag(&self) -> u8 {
        return 179;
    }
}
pub struct SecuritySettlAgentAcctName {
    value: char,
}
impl Field for SecuritySettlAgentContactName {
    fn tag(&self) -> u8 {
        return 180;
    }
}
pub struct SecuritySettlAgentContactName {
    value: char,
}
impl Field for SecuritySettlAgentContactPhone {
    fn tag(&self) -> u8 {
        return 181;
    }
}
pub struct SecuritySettlAgentContactPhone {
    value: char,
}
impl Field for CashSettlAgentName {
    fn tag(&self) -> u8 {
        return 182;
    }
}
pub struct CashSettlAgentName {
    value: char,
}
impl Field for CashSettlAgentCode {
    fn tag(&self) -> u8 {
        return 183;
    }
}
pub struct CashSettlAgentCode {
    value: char,
}
impl Field for CashSettlAgentAcctNum {
    fn tag(&self) -> u8 {
        return 184;
    }
}
pub struct CashSettlAgentAcctNum {
    value: char,
}
impl Field for CashSettlAgentAcctName {
    fn tag(&self) -> u8 {
        return 185;
    }
}
pub struct CashSettlAgentAcctName {
    value: char,
}
impl Field for CashSettlAgentContactName {
    fn tag(&self) -> u8 {
        return 186;
    }
}
pub struct CashSettlAgentContactName {
    value: char,
}
impl Field for CashSettlAgentContactPhone {
    fn tag(&self) -> u8 {
        return 187;
    }
}
pub struct CashSettlAgentContactPhone {
    value: char,
}
impl Field for BidSpotRate {
    fn tag(&self) -> u8 {
        return 188;
    }
}
pub struct BidSpotRate {
    value: f32,
}
impl Field for BidForwardPoints {
    fn tag(&self) -> u8 {
        return 189;
    }
}
pub struct BidForwardPoints {
    value: f32,
}
impl Field for OfferSpotRate {
    fn tag(&self) -> u8 {
        return 190;
    }
}
pub struct OfferSpotRate {
    value: f32,
}
impl Field for OfferForwardPoints {
    fn tag(&self) -> u8 {
        return 191;
    }
}
pub struct OfferForwardPoints {
    value: f32,
}
impl Field for OrderQty2 {
    fn tag(&self) -> u8 {
        return 192;
    }
}
pub struct OrderQty2 {
    value: f32,
}
impl Field for FutSettDate2 {
    fn tag(&self) -> u8 {
        return 193;
    }
}
pub struct FutSettDate2 {
    value: String,
}
impl Field for LastSpotRate {
    fn tag(&self) -> u8 {
        return 194;
    }
}
pub struct LastSpotRate {
    value: f32,
}
impl Field for LastForwardPoints {
    fn tag(&self) -> u8 {
        return 195;
    }
}
pub struct LastForwardPoints {
    value: f32,
}
impl Field for AllocLinkID {
    fn tag(&self) -> u8 {
        return 196;
    }
}
pub struct AllocLinkID {
    value: char,
}
impl Field for AllocLinkType {
    fn tag(&self) -> u8 {
        return 197;
    }
}
#[derive(Debug)]
pub enum AllocLinkType {
    _Fxnetting,
    _Fxswap,
}
impl Field for SecondaryOrderID {
    fn tag(&self) -> u8 {
        return 198;
    }
}
pub struct SecondaryOrderID {
    value: char,
}
impl Field for NoIOIQualifiers {
    fn tag(&self) -> u8 {
        return 199;
    }
}
pub struct NoIOIQualifiers {
    value: u16,
}
impl Field for MaturityMonthYear {
    fn tag(&self) -> u8 {
        return 200;
    }
}
pub struct MaturityMonthYear {
    value: u8,
}
impl Field for PutOrCall {
    fn tag(&self) -> u8 {
        return 201;
    }
}
#[derive(Debug)]
pub enum PutOrCall {
    _Put,
    _Call,
}
impl Field for StrikePrice {
    fn tag(&self) -> u8 {
        return 202;
    }
}
pub struct StrikePrice {
    value: f32,
}
impl Field for CoveredOrUncovered {
    fn tag(&self) -> u8 {
        return 203;
    }
}
#[derive(Debug)]
pub enum CoveredOrUncovered {
    _Covered,
    _Uncovered,
}
impl Field for CustomerOrFirm {
    fn tag(&self) -> u8 {
        return 204;
    }
}
#[derive(Debug)]
pub enum CustomerOrFirm {
    _Customer,
    _Firm,
}
impl Field for MaturityDay {
    fn tag(&self) -> u8 {
        return 205;
    }
}
pub struct MaturityDay {
    value: u8,
}
impl Field for OptAttribute {
    fn tag(&self) -> u8 {
        return 206;
    }
}
pub struct OptAttribute {
    value: char,
}
impl Field for SecurityExchange {
    fn tag(&self) -> u8 {
        return 207;
    }
}
pub struct SecurityExchange {
    value: char,
}
impl Field for NotifyBrokerOfCredit {
    fn tag(&self) -> u8 {
        return 208;
    }
}
#[derive(Debug)]
pub enum NotifyBrokerOfCredit {
    _No,
    _Yes,
}
impl Field for AllocHandlInst {
    fn tag(&self) -> u8 {
        return 209;
    }
}
#[derive(Debug)]
pub enum AllocHandlInst {
    _Match,
    _Forward,
    _Forwardandmatch,
}
impl Field for MaxShow {
    fn tag(&self) -> u8 {
        return 210;
    }
}
pub struct MaxShow {
    value: u16,
}
impl Field for PegDifference {
    fn tag(&self) -> u8 {
        return 211;
    }
}
pub struct PegDifference {
    value: f32,
}