use field::Field;

impl Field for Account {
    fn tag(&self) -> u8 {
        return 1;
    }
}
pub struct Account {
    value: String,
}
impl Field for AdvId {
    fn tag(&self) -> u8 {
        return 2;
    }
}
pub struct AdvId {
    value: String,
}
impl Field for AdvRefID {
    fn tag(&self) -> u8 {
        return 3;
    }
}
pub struct AdvRefID {
    value: String,
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
    value: String,
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
    value: String,
}
impl Field for ClOrdID {
    fn tag(&self) -> u8 {
        return 11;
    }
}
pub struct ClOrdID {
    value: String,
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
    value: f32,
}
impl Field for Currency {
    fn tag(&self) -> u8 {
        return 15;
    }
}
pub struct Currency {
    value: f32,
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
    value: String,
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
    _Fixedpegtolocalbestbidorofferattimeoforder,
    _Customerdisplayinstruction,
    _Netting,
    _Pegtovwap,
}
impl Field for ExecRefID {
    fn tag(&self) -> u8 {
        return 19;
    }
}
pub struct ExecRefID {
    value: String,
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
    _Exchangesymbol,
    _Consolidatedtapeassociation,
}
impl Field for IOIid {
    fn tag(&self) -> u8 {
        return 23;
    }
}
pub struct IOIid {
    value: String,
}
impl Field for IOIOthSvc {
    fn tag(&self) -> u8 {
        return 24;
    }
}
pub struct IOIOthSvc {
    value: char,
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
    value: String,
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
    value: String,
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
    value: f32,
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
    _Quotestatusrequest,
    _Logon,
    _News,
    _Quoteacknowledgement,
    _Email,
    _Securitydefinitionrequest,
    _Ordersingle,
    _Securitydefinition,
    _Orderlist,
    _Securitystatusrequest,
    _Securitystatus,
    _Ordercancelrequest,
    _Ordercancelreplacerequest,
    _Tradingsessionstatusrequest,
    _Orderstatusrequest,
    _Tradingsessionstatus,
    _Massquote,
    _Businessmessagereject,
    _Allocation,
    _Listcancelrequest,
    _Bidrequest,
    _Bidresponse,
    _Listexecute,
    _Liststrikeprice,
    _Liststatusrequest,
    _Liststatus,
    _Allocationack,
    _Dontknowtrade,
    _Quoterequest,
    _Quote,
    _Settlementinstructions,
    _Marketdatarequest,
    _Marketdatasnapshotfullrefresh,
    _Marketdataincrementalrefresh,
    _Marketdatarequestreject,
    _Quotecancel,
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
    value: String,
}
impl Field for OrderQty {
    fn tag(&self) -> u8 {
        return 38;
    }
}
pub struct OrderQty {
    value: f32,
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
    _Pendingcancel,
    _Stopped,
    _Rejected,
    _Suspended,
    _Pendingnew,
    _Calculated,
    _Expired,
    _Acceptedforbidding,
    _Pendingreplace,
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
    _Funari,
    _Pegged,
}
impl Field for OrigClOrdID {
    fn tag(&self) -> u8 {
        return 41;
    }
}
pub struct OrigClOrdID {
    value: String,
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
    value: String,
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
    value: String,
}
impl Field for SenderCompID {
    fn tag(&self) -> u8 {
        return 49;
    }
}
pub struct SenderCompID {
    value: String,
}
impl Field for SenderSubID {
    fn tag(&self) -> u8 {
        return 50;
    }
}
pub struct SenderSubID {
    value: String,
}
impl Field for SendingDate {
    fn tag(&self) -> u8 {
        return 51;
    }
}
pub struct SendingDate {
    value: u64,
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
    value: f32,
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
    _Crossshort,
}
impl Field for Symbol {
    fn tag(&self) -> u8 {
        return 55;
    }
}
pub struct Symbol {
    value: String,
}
impl Field for TargetCompID {
    fn tag(&self) -> u8 {
        return 56;
    }
}
pub struct TargetCompID {
    value: String,
}
impl Field for TargetSubID {
    fn tag(&self) -> u8 {
        return 57;
    }
}
pub struct TargetSubID {
    value: String,
}
impl Field for Text {
    fn tag(&self) -> u8 {
        return 58;
    }
}
pub struct Text {
    value: String,
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
    value: u64,
}
impl Field for SymbolSfx {
    fn tag(&self) -> u8 {
        return 65;
    }
}
pub struct SymbolSfx {
    value: String,
}
impl Field for ListID {
    fn tag(&self) -> u8 {
        return 66;
    }
}
pub struct ListID {
    value: String,
}
impl Field for ListSeqNo {
    fn tag(&self) -> u8 {
        return 67;
    }
}
pub struct ListSeqNo {
    value: u16,
}
impl Field for TotNoOrders {
    fn tag(&self) -> u8 {
        return 68;
    }
}
pub struct TotNoOrders {
    value: u16,
}
impl Field for ListExecInst {
    fn tag(&self) -> u8 {
        return 69;
    }
}
pub struct ListExecInst {
    value: String,
}
impl Field for AllocID {
    fn tag(&self) -> u8 {
        return 70;
    }
}
pub struct AllocID {
    value: String,
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
    _Calculatedwithoutpreliminary,
}
impl Field for RefAllocID {
    fn tag(&self) -> u8 {
        return 72;
    }
}
pub struct RefAllocID {
    value: String,
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
    value: u64,
}
impl Field for ExecBroker {
    fn tag(&self) -> u8 {
        return 76;
    }
}
pub struct ExecBroker {
    value: String,
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
    value: String,
}
impl Field for AllocShares {
    fn tag(&self) -> u8 {
        return 80;
    }
}
pub struct AllocShares {
    value: f32,
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
    value: f32,
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
    value: String,
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
    value: String,
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
    value: String,
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
    _Brokeroption,
    _Orderalreadyinpendingcancelorpendingreplacestatus,
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
    _Duplicateofaverballycommunicatedorder,
    _Staleorder,
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
    _Readytotrade,
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
    value: String,
}
impl Field for Issuer {
    fn tag(&self) -> u8 {
        return 106;
    }
}
pub struct Issuer {
    value: String,
}
impl Field for SecurityDesc {
    fn tag(&self) -> u8 {
        return 107;
    }
}
pub struct SecurityDesc {
    value: String,
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
    value: String,
}
impl Field for MinQty {
    fn tag(&self) -> u8 {
        return 110;
    }
}
pub struct MinQty {
    value: f32,
}
impl Field for MaxFloor {
    fn tag(&self) -> u8 {
        return 111;
    }
}
pub struct MaxFloor {
    value: f32,
}
impl Field for TestReqID {
    fn tag(&self) -> u8 {
        return 112;
    }
}
pub struct TestReqID {
    value: String,
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
    value: String,
}
impl Field for OnBehalfOfSubID {
    fn tag(&self) -> u8 {
        return 116;
    }
}
pub struct OnBehalfOfSubID {
    value: String,
}
impl Field for QuoteID {
    fn tag(&self) -> u8 {
        return 117;
    }
}
pub struct QuoteID {
    value: String,
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
    value: f32,
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
    value: String,
}
impl Field for DeliverToSubID {
    fn tag(&self) -> u8 {
        return 129;
    }
}
pub struct DeliverToSubID {
    value: String,
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
    value: String,
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
    value: f32,
}
impl Field for OfferSize {
    fn tag(&self) -> u8 {
        return 135;
    }
}
pub struct OfferSize {
    value: f32,
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
    value: f32,
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
    _Consumptiontax,
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
    value: String,
}
impl Field for TargetLocationID {
    fn tag(&self) -> u8 {
        return 143;
    }
}
pub struct TargetLocationID {
    value: String,
}
impl Field for OnBehalfOfLocationID {
    fn tag(&self) -> u8 {
        return 144;
    }
}
pub struct OnBehalfOfLocationID {
    value: String,
}
impl Field for DeliverToLocationID {
    fn tag(&self) -> u8 {
        return 145;
    }
}
pub struct DeliverToLocationID {
    value: String,
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
    value: String,
}
impl Field for Headline {
    fn tag(&self) -> u8 {
        return 148;
    }
}
pub struct Headline {
    value: String,
}
impl Field for URLLink {
    fn tag(&self) -> u8 {
        return 149;
    }
}
pub struct URLLink {
    value: String,
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
    _Canceled,
    _Replace,
    _Pendingcancel,
    _Stopped,
    _Rejected,
    _Suspended,
    _Pendingnew,
    _Calculated,
    _Expired,
    _Restated,
    _Pendingreplace,
}
impl Field for LeavesQty {
    fn tag(&self) -> u8 {
        return 151;
    }
}
pub struct LeavesQty {
    value: f32,
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
    value: String,
}
impl Field for SettlInstID {
    fn tag(&self) -> u8 {
        return 162;
    }
}
pub struct SettlInstID {
    value: String,
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
    value: String,
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
    _Wildcardentry,
    _Bankersacceptance,
    _Convertiblebond,
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
    _Mortgageioette,
    _Mutualfund,
    _Mortgageinterestonly,
    _Mortgageprincipalonly,
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
    value: String,
}
impl Field for StandInstDbID {
    fn tag(&self) -> u8 {
        return 171;
    }
}
pub struct StandInstDbID {
    value: String,
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
    value: String,
}
impl Field for SettlBrkrCode {
    fn tag(&self) -> u8 {
        return 174;
    }
}
pub struct SettlBrkrCode {
    value: String,
}
impl Field for SettlInstCode {
    fn tag(&self) -> u8 {
        return 175;
    }
}
pub struct SettlInstCode {
    value: String,
}
impl Field for SecuritySettlAgentName {
    fn tag(&self) -> u8 {
        return 176;
    }
}
pub struct SecuritySettlAgentName {
    value: String,
}
impl Field for SecuritySettlAgentCode {
    fn tag(&self) -> u8 {
        return 177;
    }
}
pub struct SecuritySettlAgentCode {
    value: String,
}
impl Field for SecuritySettlAgentAcctNum {
    fn tag(&self) -> u8 {
        return 178;
    }
}
pub struct SecuritySettlAgentAcctNum {
    value: String,
}
impl Field for SecuritySettlAgentAcctName {
    fn tag(&self) -> u8 {
        return 179;
    }
}
pub struct SecuritySettlAgentAcctName {
    value: String,
}
impl Field for SecuritySettlAgentContactName {
    fn tag(&self) -> u8 {
        return 180;
    }
}
pub struct SecuritySettlAgentContactName {
    value: String,
}
impl Field for SecuritySettlAgentContactPhone {
    fn tag(&self) -> u8 {
        return 181;
    }
}
pub struct SecuritySettlAgentContactPhone {
    value: String,
}
impl Field for CashSettlAgentName {
    fn tag(&self) -> u8 {
        return 182;
    }
}
pub struct CashSettlAgentName {
    value: String,
}
impl Field for CashSettlAgentCode {
    fn tag(&self) -> u8 {
        return 183;
    }
}
pub struct CashSettlAgentCode {
    value: String,
}
impl Field for CashSettlAgentAcctNum {
    fn tag(&self) -> u8 {
        return 184;
    }
}
pub struct CashSettlAgentAcctNum {
    value: String,
}
impl Field for CashSettlAgentAcctName {
    fn tag(&self) -> u8 {
        return 185;
    }
}
pub struct CashSettlAgentAcctName {
    value: String,
}
impl Field for CashSettlAgentContactName {
    fn tag(&self) -> u8 {
        return 186;
    }
}
pub struct CashSettlAgentContactName {
    value: String,
}
impl Field for CashSettlAgentContactPhone {
    fn tag(&self) -> u8 {
        return 187;
    }
}
pub struct CashSettlAgentContactPhone {
    value: String,
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
    value: i8,
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
    value: i8,
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
    value: u64,
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
    value: i8,
}
impl Field for AllocLinkID {
    fn tag(&self) -> u8 {
        return 196;
    }
}
pub struct AllocLinkID {
    value: String,
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
    value: String,
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
    value: String,
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
    value: f32,
}
impl Field for PegDifference {
    fn tag(&self) -> u8 {
        return 211;
    }
}
pub struct PegDifference {
    value: i8,
}
impl Field for XmlDataLen {
    fn tag(&self) -> u8 {
        return 212;
    }
}
pub struct XmlDataLen {
    value: usize,
}
impl Field for XmlData {
    fn tag(&self) -> u8 {
        return 213;
    }
}
pub struct XmlData {
    value: [u8; 1024],
}
impl Field for SettlInstRefID {
    fn tag(&self) -> u8 {
        return 214;
    }
}
pub struct SettlInstRefID {
    value: String,
}
impl Field for NoRoutingIDs {
    fn tag(&self) -> u8 {
        return 215;
    }
}
pub struct NoRoutingIDs {
    value: u16,
}
impl Field for RoutingType {
    fn tag(&self) -> u8 {
        return 216;
    }
}
#[derive(Debug)]
pub enum RoutingType {
    _Targetfirm,
    _Targetlist,
    _Blockfirm,
    _Blocklist,
}
impl Field for RoutingID {
    fn tag(&self) -> u8 {
        return 217;
    }
}
pub struct RoutingID {
    value: String,
}
impl Field for SpreadToBenchmark {
    fn tag(&self) -> u8 {
        return 218;
    }
}
pub struct SpreadToBenchmark {
    value: i8,
}
impl Field for Benchmark {
    fn tag(&self) -> u8 {
        return 219;
    }
}
#[derive(Debug)]
pub enum Benchmark {
    _Curve,
    _5Yr,
    _Old5,
    _10Yr,
    _Old10,
    _30Yr,
    _Old30,
    _3Molibor,
    _6Molibor,
}
impl Field for CouponRate {
    fn tag(&self) -> u8 {
        return 223;
    }
}
pub struct CouponRate {
    value: f32,
}
impl Field for ContractMultiplier {
    fn tag(&self) -> u8 {
        return 231;
    }
}
pub struct ContractMultiplier {
    value: f32,
}
impl Field for MDReqID {
    fn tag(&self) -> u8 {
        return 262;
    }
}
pub struct MDReqID {
    value: String,
}
impl Field for SubscriptionRequestType {
    fn tag(&self) -> u8 {
        return 263;
    }
}
#[derive(Debug)]
pub enum SubscriptionRequestType {
    _Snapshot,
    _Snapshotplusupdates,
    _Disableprevioussnapshotplusupdaterequest,
}
impl Field for MarketDepth {
    fn tag(&self) -> u8 {
        return 264;
    }
}
pub struct MarketDepth {
    value: u16,
}
impl Field for MDUpdateType {
    fn tag(&self) -> u8 {
        return 265;
    }
}
#[derive(Debug)]
pub enum MDUpdateType {
    _Fullrefresh,
    _Incrementalrefresh,
}
impl Field for AggregatedBook {
    fn tag(&self) -> u8 {
        return 266;
    }
}
#[derive(Debug)]
pub enum AggregatedBook {
    _No,
    _Yes,
}
impl Field for NoMDEntryTypes {
    fn tag(&self) -> u8 {
        return 267;
    }
}
pub struct NoMDEntryTypes {
    value: u16,
}
impl Field for NoMDEntries {
    fn tag(&self) -> u8 {
        return 268;
    }
}
pub struct NoMDEntries {
    value: u16,
}
impl Field for MDEntryType {
    fn tag(&self) -> u8 {
        return 269;
    }
}
#[derive(Debug)]
pub enum MDEntryType {
    _Bid,
    _Offer,
    _Trade,
    _Indexvalue,
    _Openingprice,
    _Closingprice,
    _Settlementprice,
    _Tradingsessionhighprice,
    _Tradingsessionlowprice,
    _Tradingsessionvwapprice,
}
impl Field for MDEntryPx {
    fn tag(&self) -> u8 {
        return 270;
    }
}
pub struct MDEntryPx {
    value: f32,
}
impl Field for MDEntrySize {
    fn tag(&self) -> u8 {
        return 271;
    }
}
pub struct MDEntrySize {
    value: f32,
}
impl Field for MDEntryDate {
    fn tag(&self) -> u8 {
        return 272;
    }
}
pub struct MDEntryDate {
    value: String,
}
impl Field for MDEntryTime {
    fn tag(&self) -> u8 {
        return 273;
    }
}
pub struct MDEntryTime {
    value: String,
}
impl Field for TickDirection {
    fn tag(&self) -> u8 {
        return 274;
    }
}
#[derive(Debug)]
pub enum TickDirection {
    _Plustick,
    _Zeroplustick,
    _Minustick,
    _Zerominustick,
}
impl Field for MDMkt {
    fn tag(&self) -> u8 {
        return 275;
    }
}
pub struct MDMkt {
    value: String,
}
impl Field for QuoteCondition {
    fn tag(&self) -> u8 {
        return 276;
    }
}
#[derive(Debug)]
pub enum QuoteCondition {
    _Open,
    _Closed,
    _Exchangebest,
    _Consolidatedbest,
    _Locked,
    _Crossed,
    _Depth,
    _Fasttrading,
    _Nonfirm,
}
impl Field for TradeCondition {
    fn tag(&self) -> u8 {
        return 277;
    }
}
#[derive(Debug)]
pub enum TradeCondition {
    _Cash,
    _Averagepricetrade,
    _Cashtrade,
    _Nextday,
    _Opening,
    _Intradaytradedetail,
    _Rule127Trade,
    _Rule155Trade,
    _Soldlast,
    _Nextdaytrade,
    _Opened,
    _Seller,
    _Sold,
    _Stoppedstock,
}
impl Field for MDEntryID {
    fn tag(&self) -> u8 {
        return 278;
    }
}
pub struct MDEntryID {
    value: String,
}
impl Field for MDUpdateAction {
    fn tag(&self) -> u8 {
        return 279;
    }
}
#[derive(Debug)]
pub enum MDUpdateAction {
    _New,
    _Change,
    _Delete,
}
impl Field for MDEntryRefID {
    fn tag(&self) -> u8 {
        return 280;
    }
}
pub struct MDEntryRefID {
    value: String,
}
impl Field for MDReqRejReason {
    fn tag(&self) -> u8 {
        return 281;
    }
}
#[derive(Debug)]
pub enum MDReqRejReason {
    _Unknownsymbol,
    _Duplicatemdreqid,
    _Insufficientbandwidth,
    _Insufficientpermissions,
    _Unsupportedsubscriptionrequesttype,
    _Unsupportedmarketdepth,
    _Unsupportedmdupdatetype,
    _Unsupportedaggregatedbook,
    _Unsupportedmdentrytype,
}
impl Field for MDEntryOriginator {
    fn tag(&self) -> u8 {
        return 282;
    }
}
pub struct MDEntryOriginator {
    value: String,
}
impl Field for LocationID {
    fn tag(&self) -> u8 {
        return 283;
    }
}
pub struct LocationID {
    value: String,
}
impl Field for DeskID {
    fn tag(&self) -> u8 {
        return 284;
    }
}
pub struct DeskID {
    value: String,
}
impl Field for DeleteReason {
    fn tag(&self) -> u8 {
        return 285;
    }
}
#[derive(Debug)]
pub enum DeleteReason {
    _Cancelation,
    _Error,
}
impl Field for OpenCloseSettleFlag {
    fn tag(&self) -> u8 {
        return 286;
    }
}
#[derive(Debug)]
pub enum OpenCloseSettleFlag {
    _Dailyopen,
    _Sessionopen,
    _Deliverysettlementprice,
}
impl Field for SellerDays {
    fn tag(&self) -> u8 {
        return 287;
    }
}
pub struct SellerDays {
    value: u16,
}
impl Field for MDEntryBuyer {
    fn tag(&self) -> u8 {
        return 288;
    }
}
pub struct MDEntryBuyer {
    value: String,
}
impl Field for MDEntrySeller {
    fn tag(&self) -> u8 {
        return 289;
    }
}
pub struct MDEntrySeller {
    value: String,
}
impl Field for MDEntryPositionNo {
    fn tag(&self) -> u8 {
        return 290;
    }
}
pub struct MDEntryPositionNo {
    value: u16,
}
impl Field for FinancialStatus {
    fn tag(&self) -> u8 {
        return 291;
    }
}
#[derive(Debug)]
pub enum FinancialStatus {
    _Bankrupt,
}
impl Field for CorporateAction {
    fn tag(&self) -> u8 {
        return 292;
    }
}
#[derive(Debug)]
pub enum CorporateAction {
    _Exdividend,
    _Exdistribution,
    _Exrights,
    _New,
    _Exinterest,
}
impl Field for DefBidSize {
    fn tag(&self) -> u8 {
        return 293;
    }
}
pub struct DefBidSize {
    value: f32,
}
impl Field for DefOfferSize {
    fn tag(&self) -> u8 {
        return 294;
    }
}
pub struct DefOfferSize {
    value: f32,
}
impl Field for NoQuoteEntries {
    fn tag(&self) -> u8 {
        return 295;
    }
}
pub struct NoQuoteEntries {
    value: u16,
}
impl Field for NoQuoteSets {
    fn tag(&self) -> u8 {
        return 296;
    }
}
pub struct NoQuoteSets {
    value: u16,
}
impl Field for QuoteAckStatus {
    fn tag(&self) -> u8 {
        return 297;
    }
}
#[derive(Debug)]
pub enum QuoteAckStatus {
    _Accepted,
    _Canceledforsymbol,
    _Canceledforsecuritytype,
    _Canceledforunderlying,
    _Canceledall,
    _Rejected,
}
impl Field for QuoteCancelType {
    fn tag(&self) -> u8 {
        return 298;
    }
}
#[derive(Debug)]
pub enum QuoteCancelType {
    _Cancelforsymbol,
    _Cancelforsecuritytype,
    _Cancelforunderlyingsymbol,
    _Cancelforallquotes,
}
impl Field for QuoteEntryID {
    fn tag(&self) -> u8 {
        return 299;
    }
}
pub struct QuoteEntryID {
    value: String,
}
impl Field for QuoteRejectReason {
    fn tag(&self) -> u8 {
        return 300;
    }
}
#[derive(Debug)]
pub enum QuoteRejectReason {
    _Unknownsymbol,
    _Exchange,
    _Quoterequestexceedslimit,
    _Toolatetoenter,
    _Unknownquote,
    _Duplicatequote,
    _Invalidbidaskspread,
    _Invalidprice,
    _Notauthorizedtoquotesecurity,
}
impl Field for QuoteResponseLevel {
    fn tag(&self) -> u8 {
        return 301;
    }
}
#[derive(Debug)]
pub enum QuoteResponseLevel {
    _Noacknowledgement,
    _Acknowledgeonlynegativeorerroneousquotes,
    _Acknowledgeeachquotemessages,
}
impl Field for QuoteSetID {
    fn tag(&self) -> u8 {
        return 302;
    }
}
pub struct QuoteSetID {
    value: String,
}
impl Field for QuoteRequestType {
    fn tag(&self) -> u8 {
        return 303;
    }
}
#[derive(Debug)]
pub enum QuoteRequestType {
    _Manual,
    _Automatic,
}
impl Field for TotQuoteEntries {
    fn tag(&self) -> u8 {
        return 304;
    }
}
pub struct TotQuoteEntries {
    value: u16,
}
impl Field for UnderlyingIDSource {
    fn tag(&self) -> u8 {
        return 305;
    }
}
pub struct UnderlyingIDSource {
    value: String,
}
impl Field for UnderlyingIssuer {
    fn tag(&self) -> u8 {
        return 306;
    }
}
pub struct UnderlyingIssuer {
    value: String,
}
impl Field for UnderlyingSecurityDesc {
    fn tag(&self) -> u8 {
        return 307;
    }
}
pub struct UnderlyingSecurityDesc {
    value: String,
}
impl Field for UnderlyingSecurityExchange {
    fn tag(&self) -> u8 {
        return 308;
    }
}
pub struct UnderlyingSecurityExchange {
    value: String,
}
impl Field for UnderlyingSecurityID {
    fn tag(&self) -> u8 {
        return 309;
    }
}
pub struct UnderlyingSecurityID {
    value: String,
}
impl Field for UnderlyingSecurityType {
    fn tag(&self) -> u8 {
        return 310;
    }
}
pub struct UnderlyingSecurityType {
    value: String,
}
impl Field for UnderlyingSymbol {
    fn tag(&self) -> u8 {
        return 311;
    }
}
pub struct UnderlyingSymbol {
    value: String,
}
impl Field for UnderlyingSymbolSfx {
    fn tag(&self) -> u8 {
        return 312;
    }
}
pub struct UnderlyingSymbolSfx {
    value: String,
}
impl Field for UnderlyingMaturityMonthYear {
    fn tag(&self) -> u8 {
        return 313;
    }
}
pub struct UnderlyingMaturityMonthYear {
    value: u8,
}
impl Field for UnderlyingMaturityDay {
    fn tag(&self) -> u8 {
        return 314;
    }
}
pub struct UnderlyingMaturityDay {
    value: u8,
}
impl Field for UnderlyingPutOrCall {
    fn tag(&self) -> u8 {
        return 315;
    }
}
pub struct UnderlyingPutOrCall {
    value: u16,
}
impl Field for UnderlyingStrikePrice {
    fn tag(&self) -> u8 {
        return 316;
    }
}
pub struct UnderlyingStrikePrice {
    value: f32,
}
impl Field for UnderlyingOptAttribute {
    fn tag(&self) -> u8 {
        return 317;
    }
}
pub struct UnderlyingOptAttribute {
    value: char,
}
impl Field for UnderlyingCurrency {
    fn tag(&self) -> u8 {
        return 318;
    }
}
pub struct UnderlyingCurrency {
    value: f32,
}
impl Field for RatioQty {
    fn tag(&self) -> u8 {
        return 319;
    }
}
pub struct RatioQty {
    value: f64,
}
impl Field for SecurityReqID {
    fn tag(&self) -> u8 {
        return 320;
    }
}
pub struct SecurityReqID {
    value: String,
}
impl Field for SecurityRequestType {
    fn tag(&self) -> u8 {
        return 321;
    }
}
#[derive(Debug)]
pub enum SecurityRequestType {
    _Requestsecurityidentityandspecifications,
    _Requestsecurityidentityforthespecificationsprovided,
    _Requestlistsecuritytypes,
    _Requestlistsecurities,
}
impl Field for SecurityResponseID {
    fn tag(&self) -> u8 {
        return 322;
    }
}
pub struct SecurityResponseID {
    value: String,
}
impl Field for SecurityResponseType {
    fn tag(&self) -> u8 {
        return 323;
    }
}
#[derive(Debug)]
pub enum SecurityResponseType {
    _Acceptsecurityproposalasis,
    _Acceptsecurityproposalwithrevisionsasindicatedinthemessage,
    _Listofsecuritytypesreturnedperrequest,
    _Listofsecuritiesreturnedperrequest,
    _Rejectsecurityproposal,
    _Cannotmatchselectioncriteria,
}
impl Field for SecurityStatusReqID {
    fn tag(&self) -> u8 {
        return 324;
    }
}
pub struct SecurityStatusReqID {
    value: String,
}
impl Field for UnsolicitedIndicator {
    fn tag(&self) -> u8 {
        return 325;
    }
}
#[derive(Debug)]
pub enum UnsolicitedIndicator {
    _No,
    _Yes,
}
impl Field for SecurityTradingStatus {
    fn tag(&self) -> u8 {
        return 326;
    }
}
#[derive(Debug)]
pub enum SecurityTradingStatus {
    _Openingdelay,
    _Marketoncloseimbalancesell,
    _11,
    _Nomarketimbalance,
    _Nomarketoncloseimbalance,
    _Itspreopening,
    _Newpriceindication,
    _Tradedisseminationtime,
    _Readytotrade,
    _Notavailablefortrading,
    _Nottradedonthismarket,
    _Tradinghalt,
    _Unknownorinvalid,
    _Resume,
    _Noopennoresume,
    _Priceindication,
    _Tradingrangeindication,
    _Marketimbalancebuy,
    _Marketimbalancesell,
    _Marketoncloseimbalancebuy,
}
impl Field for HaltReasonChar {
    fn tag(&self) -> u8 {
        return 327;
    }
}
#[derive(Debug)]
pub enum HaltReasonChar {
    _Newsdissemination,
    _Orderinflux,
    _Orderimbalance,
    _Additionalinformation,
    _Newspending,
    _Equipmentchangeover,
}
impl Field for InViewOfCommon {
    fn tag(&self) -> u8 {
        return 328;
    }
}
#[derive(Debug)]
pub enum InViewOfCommon {
    _No,
    _Yes,
}
impl Field for DueToRelated {
    fn tag(&self) -> u8 {
        return 329;
    }
}
#[derive(Debug)]
pub enum DueToRelated {
    _No,
    _Yes,
}
impl Field for BuyVolume {
    fn tag(&self) -> u8 {
        return 330;
    }
}
pub struct BuyVolume {
    value: f32,
}
impl Field for SellVolume {
    fn tag(&self) -> u8 {
        return 331;
    }
}
pub struct SellVolume {
    value: f32,
}
impl Field for HighPx {
    fn tag(&self) -> u8 {
        return 332;
    }
}
pub struct HighPx {
    value: f32,
}
impl Field for LowPx {
    fn tag(&self) -> u8 {
        return 333;
    }
}
pub struct LowPx {
    value: f32,
}
impl Field for Adjustment {
    fn tag(&self) -> u8 {
        return 334;
    }
}
#[derive(Debug)]
pub enum Adjustment {
    _Cancel,
    _Error,
    _Correction,
}
impl Field for TradSesReqID {
    fn tag(&self) -> u8 {
        return 335;
    }
}
pub struct TradSesReqID {
    value: String,
}
impl Field for TradingSessionID {
    fn tag(&self) -> u8 {
        return 336;
    }
}
pub struct TradingSessionID {
    value: String,
}
impl Field for ContraTrader {
    fn tag(&self) -> u8 {
        return 337;
    }
}
pub struct ContraTrader {
    value: String,
}
impl Field for TradSesMethod {
    fn tag(&self) -> u8 {
        return 338;
    }
}
#[derive(Debug)]
pub enum TradSesMethod {
    _Electronic,
    _Openoutcry,
    _Twoparty,
}
impl Field for TradSesMode {
    fn tag(&self) -> u8 {
        return 339;
    }
}
#[derive(Debug)]
pub enum TradSesMode {
    _Testing,
    _Simulated,
    _Production,
}
impl Field for TradSesStatus {
    fn tag(&self) -> u8 {
        return 340;
    }
}
#[derive(Debug)]
pub enum TradSesStatus {
    _Halted,
    _Open,
    _Closed,
    _Preopen,
    _Preclose,
}
impl Field for TradSesStartTime {
    fn tag(&self) -> u8 {
        return 341;
    }
}
pub struct TradSesStartTime {
    value: u64,
}
impl Field for TradSesOpenTime {
    fn tag(&self) -> u8 {
        return 342;
    }
}
pub struct TradSesOpenTime {
    value: u64,
}
impl Field for TradSesPreCloseTime {
    fn tag(&self) -> u8 {
        return 343;
    }
}
pub struct TradSesPreCloseTime {
    value: u64,
}
impl Field for TradSesCloseTime {
    fn tag(&self) -> u8 {
        return 344;
    }
}
pub struct TradSesCloseTime {
    value: u64,
}
impl Field for TradSesEndTime {
    fn tag(&self) -> u8 {
        return 345;
    }
}
pub struct TradSesEndTime {
    value: u64,
}
impl Field for NumberOfOrders {
    fn tag(&self) -> u8 {
        return 346;
    }
}
pub struct NumberOfOrders {
    value: u16,
}
impl Field for MessageEncoding {
    fn tag(&self) -> u8 {
        return 347;
    }
}
#[derive(Debug)]
pub enum MessageEncoding {
    _Eucjp,
    _Iso2022Jp,
    _Shiftjis,
    _Utf8,
}
impl Field for EncodedIssuerLen {
    fn tag(&self) -> u8 {
        return 348;
    }
}
pub struct EncodedIssuerLen {
    value: usize,
}
impl Field for EncodedIssuer {
    fn tag(&self) -> u8 {
        return 349;
    }
}
pub struct EncodedIssuer {
    value: [u8; 1024],
}
impl Field for EncodedSecurityDescLen {
    fn tag(&self) -> u8 {
        return 350;
    }
}
pub struct EncodedSecurityDescLen {
    value: usize,
}
impl Field for EncodedSecurityDesc {
    fn tag(&self) -> u8 {
        return 351;
    }
}
pub struct EncodedSecurityDesc {
    value: [u8; 1024],
}
impl Field for EncodedListExecInstLen {
    fn tag(&self) -> u8 {
        return 352;
    }
}
pub struct EncodedListExecInstLen {
    value: usize,
}
impl Field for EncodedListExecInst {
    fn tag(&self) -> u8 {
        return 353;
    }
}
pub struct EncodedListExecInst {
    value: [u8; 1024],
}
impl Field for EncodedTextLen {
    fn tag(&self) -> u8 {
        return 354;
    }
}
pub struct EncodedTextLen {
    value: usize,
}
impl Field for EncodedText {
    fn tag(&self) -> u8 {
        return 355;
    }
}
pub struct EncodedText {
    value: [u8; 1024],
}
impl Field for EncodedSubjectLen {
    fn tag(&self) -> u8 {
        return 356;
    }
}
pub struct EncodedSubjectLen {
    value: usize,
}
impl Field for EncodedSubject {
    fn tag(&self) -> u8 {
        return 357;
    }
}
pub struct EncodedSubject {
    value: [u8; 1024],
}
impl Field for EncodedHeadlineLen {
    fn tag(&self) -> u8 {
        return 358;
    }
}
pub struct EncodedHeadlineLen {
    value: usize,
}
impl Field for EncodedHeadline {
    fn tag(&self) -> u8 {
        return 359;
    }
}
pub struct EncodedHeadline {
    value: [u8; 1024],
}
impl Field for EncodedAllocTextLen {
    fn tag(&self) -> u8 {
        return 360;
    }
}
pub struct EncodedAllocTextLen {
    value: usize,
}
impl Field for EncodedAllocText {
    fn tag(&self) -> u8 {
        return 361;
    }
}
pub struct EncodedAllocText {
    value: [u8; 1024],
}
impl Field for EncodedUnderlyingIssuerLen {
    fn tag(&self) -> u8 {
        return 362;
    }
}
pub struct EncodedUnderlyingIssuerLen {
    value: usize,
}
impl Field for EncodedUnderlyingIssuer {
    fn tag(&self) -> u8 {
        return 363;
    }
}
pub struct EncodedUnderlyingIssuer {
    value: [u8; 1024],
}
impl Field for EncodedUnderlyingSecurityDescLen {
    fn tag(&self) -> u8 {
        return 364;
    }
}
pub struct EncodedUnderlyingSecurityDescLen {
    value: usize,
}
impl Field for EncodedUnderlyingSecurityDesc {
    fn tag(&self) -> u8 {
        return 365;
    }
}
pub struct EncodedUnderlyingSecurityDesc {
    value: [u8; 1024],
}
impl Field for AllocPrice {
    fn tag(&self) -> u8 {
        return 366;
    }
}
pub struct AllocPrice {
    value: f32,
}
impl Field for QuoteSetValidUntilTime {
    fn tag(&self) -> u8 {
        return 367;
    }
}
pub struct QuoteSetValidUntilTime {
    value: u64,
}
impl Field for QuoteEntryRejectReason {
    fn tag(&self) -> u8 {
        return 368;
    }
}
#[derive(Debug)]
pub enum QuoteEntryRejectReason {
    _Unknownsymbol,
    _Exchange,
    _Quoteexceedslimit,
    _Toolatetoenter,
    _Unknownquote,
    _Duplicatequote,
    _Invalidbidaskspread,
    _Invalidprice,
    _Notauthorizedtoquotesecurity,
}
impl Field for LastMsgSeqNumProcessed {
    fn tag(&self) -> u8 {
        return 369;
    }
}
pub struct LastMsgSeqNumProcessed {
    value: u16,
}
impl Field for OnBehalfOfSendingTime {
    fn tag(&self) -> u8 {
        return 370;
    }
}
pub struct OnBehalfOfSendingTime {
    value: u64,
}
impl Field for RefTagID {
    fn tag(&self) -> u8 {
        return 371;
    }
}
pub struct RefTagID {
    value: u16,
}
impl Field for RefMsgType {
    fn tag(&self) -> u8 {
        return 372;
    }
}
pub struct RefMsgType {
    value: String,
}
impl Field for SessionRejectReason {
    fn tag(&self) -> u8 {
        return 373;
    }
}
#[derive(Debug)]
pub enum SessionRejectReason {
    _Invalidtagnumber,
    _Requiredtagmissing,
    _Sendingtimeaccuracyproblem,
    _Invalidmsgtype,
    _Tagnotdefinedforthismessagetype,
    _Undefinedtag,
    _Tagspecifiedwithoutavalue,
    _Valueisincorrect,
    _Incorrectdataformatforvalue,
    _Decryptionproblem,
    _Signatureproblem,
    _Compidproblem,
}
impl Field for BidRequestTransType {
    fn tag(&self) -> u8 {
        return 374;
    }
}
#[derive(Debug)]
pub enum BidRequestTransType {
    _Cancel,
    _No,
}
impl Field for ContraBroker {
    fn tag(&self) -> u8 {
        return 375;
    }
}
pub struct ContraBroker {
    value: String,
}
impl Field for ComplianceID {
    fn tag(&self) -> u8 {
        return 376;
    }
}
pub struct ComplianceID {
    value: String,
}
impl Field for SolicitedFlag {
    fn tag(&self) -> u8 {
        return 377;
    }
}
#[derive(Debug)]
pub enum SolicitedFlag {
    _No,
    _Yes,
}
impl Field for ExecRestatementReason {
    fn tag(&self) -> u8 {
        return 378;
    }
}
#[derive(Debug)]
pub enum ExecRestatementReason {
    _Gtcorporateaction,
    _Gtrenewal,
    _Verbalchange,
    _Repricingoforder,
    _Brokeroption,
    _Partialdeclineoforderqty,
}
impl Field for BusinessRejectRefID {
    fn tag(&self) -> u8 {
        return 379;
    }
}
pub struct BusinessRejectRefID {
    value: String,
}
impl Field for BusinessRejectReason {
    fn tag(&self) -> u8 {
        return 380;
    }
}
#[derive(Debug)]
pub enum BusinessRejectReason {
    _Other,
    _Unkownid,
    _Unknownsecurity,
    _Unsupportedmessagetype,
    _Applicationnotavailable,
    _Conditionallyrequiredfieldmissing,
}
impl Field for GrossTradeAmt {
    fn tag(&self) -> u8 {
        return 381;
    }
}
pub struct GrossTradeAmt {
    value: f32,
}
impl Field for NoContraBrokers {
    fn tag(&self) -> u8 {
        return 382;
    }
}
pub struct NoContraBrokers {
    value: u16,
}
impl Field for MaxMessageSize {
    fn tag(&self) -> u8 {
        return 383;
    }
}
pub struct MaxMessageSize {
    value: u16,
}
impl Field for NoMsgTypes {
    fn tag(&self) -> u8 {
        return 384;
    }
}
pub struct NoMsgTypes {
    value: u16,
}
impl Field for MsgDirection {
    fn tag(&self) -> u8 {
        return 385;
    }
}
#[derive(Debug)]
pub enum MsgDirection {
    _Receive,
    _Send,
}
impl Field for NoTradingSessions {
    fn tag(&self) -> u8 {
        return 386;
    }
}
pub struct NoTradingSessions {
    value: u16,
}
impl Field for TotalVolumeTraded {
    fn tag(&self) -> u8 {
        return 387;
    }
}
pub struct TotalVolumeTraded {
    value: f32,
}
impl Field for DiscretionInst {
    fn tag(&self) -> u8 {
        return 388;
    }
}
#[derive(Debug)]
pub enum DiscretionInst {
    _Relatedtodisplayedprice,
    _Relatedtomarketprice,
    _Relatedtoprimaryprice,
    _Relatedtolocalprimaryprice,
    _Relatedtomidpointprice,
    _Relatedtolasttradeprice,
}
impl Field for DiscretionOffset {
    fn tag(&self) -> u8 {
        return 389;
    }
}
pub struct DiscretionOffset {
    value: i8,
}
impl Field for BidID {
    fn tag(&self) -> u8 {
        return 390;
    }
}
pub struct BidID {
    value: String,
}
impl Field for ClientBidID {
    fn tag(&self) -> u8 {
        return 391;
    }
}
pub struct ClientBidID {
    value: String,
}
impl Field for ListName {
    fn tag(&self) -> u8 {
        return 392;
    }
}
pub struct ListName {
    value: String,
}
impl Field for TotalNumSecurities {
    fn tag(&self) -> u8 {
        return 393;
    }
}
pub struct TotalNumSecurities {
    value: u16,
}
impl Field for BidType {
    fn tag(&self) -> u8 {
        return 394;
    }
}
pub struct BidType {
    value: u16,
}
impl Field for NumTickets {
    fn tag(&self) -> u8 {
        return 395;
    }
}
pub struct NumTickets {
    value: u16,
}
impl Field for SideValue1 {
    fn tag(&self) -> u8 {
        return 396;
    }
}
pub struct SideValue1 {
    value: f32,
}
impl Field for SideValue2 {
    fn tag(&self) -> u8 {
        return 397;
    }
}
pub struct SideValue2 {
    value: f32,
}
impl Field for NoBidDescriptors {
    fn tag(&self) -> u8 {
        return 398;
    }
}
pub struct NoBidDescriptors {
    value: u16,
}
impl Field for BidDescriptorType {
    fn tag(&self) -> u8 {
        return 399;
    }
}
pub struct BidDescriptorType {
    value: u16,
}
impl Field for BidDescriptor {
    fn tag(&self) -> u8 {
        return 400;
    }
}
pub struct BidDescriptor {
    value: String,
}
impl Field for SideValueInd {
    fn tag(&self) -> u8 {
        return 401;
    }
}
pub struct SideValueInd {
    value: u16,
}
impl Field for LiquidityPctLow {
    fn tag(&self) -> u8 {
        return 402;
    }
}
pub struct LiquidityPctLow {
    value: f32,
}
impl Field for LiquidityPctHigh {
    fn tag(&self) -> u8 {
        return 403;
    }
}
pub struct LiquidityPctHigh {
    value: f32,
}
impl Field for LiquidityValue {
    fn tag(&self) -> u8 {
        return 404;
    }
}
pub struct LiquidityValue {
    value: f32,
}
impl Field for EFPTrackingError {
    fn tag(&self) -> u8 {
        return 405;
    }
}
pub struct EFPTrackingError {
    value: f32,
}
impl Field for FairValue {
    fn tag(&self) -> u8 {
        return 406;
    }
}
pub struct FairValue {
    value: f32,
}
impl Field for OutsideIndexPct {
    fn tag(&self) -> u8 {
        return 407;
    }
}
pub struct OutsideIndexPct {
    value: f32,
}
impl Field for ValueOfFutures {
    fn tag(&self) -> u8 {
        return 408;
    }
}
pub struct ValueOfFutures {
    value: f32,
}
impl Field for LiquidityIndType {
    fn tag(&self) -> u8 {
        return 409;
    }
}
#[derive(Debug)]
pub enum LiquidityIndType {
    _5Daymovingaverage,
    _20Daymovingaverage,
    _Normalmarketsize,
    _Other,
}
impl Field for WtAverageLiquidity {
    fn tag(&self) -> u8 {
        return 410;
    }
}
pub struct WtAverageLiquidity {
    value: f32,
}
impl Field for ExchangeForPhysical {
    fn tag(&self) -> u8 {
        return 411;
    }
}
#[derive(Debug)]
pub enum ExchangeForPhysical {
    _No,
    _Yes,
}
impl Field for OutMainCntryUIndex {
    fn tag(&self) -> u8 {
        return 412;
    }
}
pub struct OutMainCntryUIndex {
    value: f32,
}
impl Field for CrossPercent {
    fn tag(&self) -> u8 {
        return 413;
    }
}
pub struct CrossPercent {
    value: f32,
}
impl Field for ProgRptReqs {
    fn tag(&self) -> u8 {
        return 414;
    }
}
#[derive(Debug)]
pub enum ProgRptReqs {
    _Buysideexplicitlyrequestsstatususingstatusrequest,
    _Sellsideperiodicallysendsstatususingliststatusperiodoptionallyspecifiedinprogressperiod,
    _Realtimeexecutionreports,
}
impl Field for ProgPeriodInterval {
    fn tag(&self) -> u8 {
        return 415;
    }
}
pub struct ProgPeriodInterval {
    value: u16,
}
impl Field for IncTaxInd {
    fn tag(&self) -> u8 {
        return 416;
    }
}
#[derive(Debug)]
pub enum IncTaxInd {
    _Net,
    _Gross,
}
impl Field for NumBidders {
    fn tag(&self) -> u8 {
        return 417;
    }
}
pub struct NumBidders {
    value: u16,
}
impl Field for TradeType {
    fn tag(&self) -> u8 {
        return 418;
    }
}
#[derive(Debug)]
pub enum TradeType {
    _Agency,
    _Vwapguarantee,
    _Guaranteedclose,
    _Risktrade,
}
impl Field for BasisPxType {
    fn tag(&self) -> u8 {
        return 419;
    }
}
#[derive(Debug)]
pub enum BasisPxType {
    _Closingpriceatmorningsession,
    _Closingprice,
    _Currentprice,
    _Sq,
    _Vwapthroughaday,
    _Vwapthroughamorningsession,
    _Vwapthroughanafternoonsession,
    _Vwapthroughadayexceptyori,
    _Vwapthroughamorningsessionexceptyori,
    _Vwapthroughanafternoonsessionexceptyori,
    _Strike,
    _Open,
    _Others,
}
impl Field for NoBidComponents {
    fn tag(&self) -> u8 {
        return 420;
    }
}
pub struct NoBidComponents {
    value: u16,
}
impl Field for Country {
    fn tag(&self) -> u8 {
        return 421;
    }
}
pub struct Country {
    value: String,
}
impl Field for TotNoStrikes {
    fn tag(&self) -> u8 {
        return 422;
    }
}
pub struct TotNoStrikes {
    value: u16,
}
impl Field for PriceType {
    fn tag(&self) -> u8 {
        return 423;
    }
}
#[derive(Debug)]
pub enum PriceType {
    _Percentage,
    _Pershare,
    _Fixedamount,
}
impl Field for DayOrderQty {
    fn tag(&self) -> u8 {
        return 424;
    }
}
pub struct DayOrderQty {
    value: f32,
}
impl Field for DayCumQty {
    fn tag(&self) -> u8 {
        return 425;
    }
}
pub struct DayCumQty {
    value: f32,
}
impl Field for DayAvgPx {
    fn tag(&self) -> u8 {
        return 426;
    }
}
pub struct DayAvgPx {
    value: f32,
}
impl Field for GTBookingInst {
    fn tag(&self) -> u8 {
        return 427;
    }
}
#[derive(Debug)]
pub enum GTBookingInst {
    _Bookoutalltradesondayofexecution,
    _Accumulateexecutionsuntilorderisfilledorexpires,
    _Accumulateuntilverballynotifiedotherwise,
}
impl Field for NoStrikes {
    fn tag(&self) -> u8 {
        return 428;
    }
}
pub struct NoStrikes {
    value: u16,
}
impl Field for ListStatusType {
    fn tag(&self) -> u8 {
        return 429;
    }
}
pub struct ListStatusType {
    value: u16,
}
impl Field for NetGrossInd {
    fn tag(&self) -> u8 {
        return 430;
    }
}
#[derive(Debug)]
pub enum NetGrossInd {
    _Net,
    _Gross,
}
impl Field for ListOrderStatus {
    fn tag(&self) -> u8 {
        return 431;
    }
}
pub struct ListOrderStatus {
    value: u16,
}
impl Field for ExpireDate {
    fn tag(&self) -> u8 {
        return 432;
    }
}
pub struct ExpireDate {
    value: u64,
}
impl Field for ListExecInstType {
    fn tag(&self) -> u8 {
        return 433;
    }
}
#[derive(Debug)]
pub enum ListExecInstType {
    _Immediate,
    _Waitforexecuteinstruction,
}
impl Field for CxlRejResponseTo {
    fn tag(&self) -> u8 {
        return 434;
    }
}
#[derive(Debug)]
pub enum CxlRejResponseTo {
    _Ordercancelrequest,
    _Ordercancelreplacerequest,
}
impl Field for UnderlyingCouponRate {
    fn tag(&self) -> u8 {
        return 435;
    }
}
pub struct UnderlyingCouponRate {
    value: f32,
}
impl Field for UnderlyingContractMultiplier {
    fn tag(&self) -> u8 {
        return 436;
    }
}
pub struct UnderlyingContractMultiplier {
    value: f32,
}
impl Field for ContraTradeQty {
    fn tag(&self) -> u8 {
        return 437;
    }
}
pub struct ContraTradeQty {
    value: f32,
}
impl Field for ContraTradeTime {
    fn tag(&self) -> u8 {
        return 438;
    }
}
pub struct ContraTradeTime {
    value: u64,
}
impl Field for ClearingFirm {
    fn tag(&self) -> u8 {
        return 439;
    }
}
pub struct ClearingFirm {
    value: String,
}
impl Field for ClearingAccount {
    fn tag(&self) -> u8 {
        return 440;
    }
}
pub struct ClearingAccount {
    value: String,
}
impl Field for LiquidityNumSecurities {
    fn tag(&self) -> u8 {
        return 441;
    }
}
pub struct LiquidityNumSecurities {
    value: u16,
}
impl Field for MultiLegReportingType {
    fn tag(&self) -> u8 {
        return 442;
    }
}
#[derive(Debug)]
pub enum MultiLegReportingType {
    _Singlesecurity,
    _Individuallegofamultilegsecurity,
    _Multilegsecurity,
}
impl Field for StrikeTime {
    fn tag(&self) -> u8 {
        return 443;
    }
}
pub struct StrikeTime {
    value: u64,
}
impl Field for ListStatusText {
    fn tag(&self) -> u8 {
        return 444;
    }
}
pub struct ListStatusText {
    value: String,
}
impl Field for EncodedListStatusTextLen {
    fn tag(&self) -> u8 {
        return 445;
    }
}
pub struct EncodedListStatusTextLen {
    value: usize,
}
impl Field for EncodedListStatusText {
    fn tag(&self) -> u8 {
        return 446;
    }
}
pub struct EncodedListStatusText {
    value: [u8; 1024],
}