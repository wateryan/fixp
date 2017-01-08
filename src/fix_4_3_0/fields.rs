use field::Field;

impl Field for Account {
    fn tag(&self) -> u16 {
        return 1;
    }
}
pub struct Account {
    value: String,
}
impl Field for AdvId {
    fn tag(&self) -> u16 {
        return 2;
    }
}
pub struct AdvId {
    value: String,
}
impl Field for AdvRefID {
    fn tag(&self) -> u16 {
        return 3;
    }
}
pub struct AdvRefID {
    value: String,
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
    _Cross,
    _Trade,
}
impl Field for AdvTransType {
    fn tag(&self) -> u16 {
        return 5;
    }
}
#[derive(Debug)]
pub enum AdvTransType {
    _New,
    _Cancel,
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
    value: u64,
}
impl Field for BeginString {
    fn tag(&self) -> u16 {
        return 8;
    }
}
pub struct BeginString {
    value: String,
}
impl Field for BodyLength {
    fn tag(&self) -> u16 {
        return 9;
    }
}
pub struct BodyLength {
    value: usize,
}
impl Field for CheckSum {
    fn tag(&self) -> u16 {
        return 10;
    }
}
pub struct CheckSum {
    value: String,
}
impl Field for ClOrdID {
    fn tag(&self) -> u16 {
        return 11;
    }
}
pub struct ClOrdID {
    value: String,
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
    _Perbond,
    _Pershare,
    _Percentage,
    _Absolute,
    _5,
    _4,
}
impl Field for CumQty {
    fn tag(&self) -> u16 {
        return 14;
    }
}
pub struct CumQty {
    value: f32,
}
impl Field for Currency {
    fn tag(&self) -> u16 {
        return 15;
    }
}
pub struct Currency {
    value: f32,
}
impl Field for EndSeqNo {
    fn tag(&self) -> u16 {
        return 16;
    }
}
pub struct EndSeqNo {
    value: u64,
}
impl Field for ExecID {
    fn tag(&self) -> u16 {
        return 17;
    }
}
pub struct ExecID {
    value: String,
}
impl Field for ExecInst {
    fn tag(&self) -> u16 {
        return 18;
    }
}
#[derive(Debug)]
pub enum ExecInst {
    _Trytostop,
    _Midprcpeg,
    _Markpeg,
    _Cancelonsysfail,
    _Primpeg,
    _Suspend,
    _Custdispinst,
    _Netting,
    _Pegvwap,
    _Tradealong,
    _Percvol,
    _Stayoffer,
    _Work,
    _Overday,
    _Held,
    _Partnotinit,
    _Strictscale,
    _Trytoscale,
    _Staybid,
    _Nocross,
    _Openpeg,
    _Callfirst,
    _Nonnego,
    _Dni,
    _Dnr,
    _Aon,
    _Restateonsysfail,
    _Institonly,
    _Restateontradinghalt,
    _Cancelontradinghalt,
    _Lastpeg,
    _Goalong,
    _Okcross,
    _Notheld,
}
impl Field for ExecRefID {
    fn tag(&self) -> u16 {
        return 19;
    }
}
pub struct ExecRefID {
    value: String,
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
impl Field for SecurityIDSource {
    fn tag(&self) -> u16 {
        return 22;
    }
}
#[derive(Debug)]
pub enum SecurityIDSource {
    _Sicovam,
    _Sedol,
    _Cusip,
    _Quik,
    _Belgian,
    _Valoren,
    _Dutch,
    _Wertpapier,
    _Bloombergsymbol,
    _Consolidatedtapeassociation,
    _Exchangesymbol,
    _Isocountrycode,
    _Isocurrencycode,
    _Riccode,
    _Isinnumber,
    _Common,
}
impl Field for IOIid {
    fn tag(&self) -> u16 {
        return 23;
    }
}
pub struct IOIid {
    value: String,
}
impl Field for IOIQltyInd {
    fn tag(&self) -> u16 {
        return 25;
    }
}
#[derive(Debug)]
pub enum IOIQltyInd {
    _Medium,
    _High,
    _Low,
}
impl Field for IOIRefID {
    fn tag(&self) -> u16 {
        return 26;
    }
}
pub struct IOIRefID {
    value: String,
}
impl Field for IOIQty {
    fn tag(&self) -> u16 {
        return 27;
    }
}
#[derive(Debug)]
pub enum IOIQty {
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
    _Principal,
    _Crossasprincipal,
    _Agent,
    _Crossasagent,
}
impl Field for LastMkt {
    fn tag(&self) -> u16 {
        return 30;
    }
}
pub struct LastMkt {
    value: String,
}
impl Field for LastPx {
    fn tag(&self) -> u16 {
        return 31;
    }
}
pub struct LastPx {
    value: f32,
}
impl Field for LastQty {
    fn tag(&self) -> u16 {
        return 32;
    }
}
pub struct LastQty {
    value: f32,
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
    value: u64,
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
    _Quotestatusrequest,
    _Logon,
    _Derivativesecuritylist,
    _Neworderab,
    _Multilegordercancelreplace,
    _Tradecapturereportrequest,
    _Tradecapturereport,
    _Ordermassstatusrequest,
    _Quoterequestreject,
    _Rfqrequest,
    _Quotestatusreport,
    _Massquoteacknowledgement,
    _News,
    _Securitydefinitionrequest,
    _Email,
    _Securitydefinition,
    _Ordersingle,
    _Securitystatusrequest,
    _Orderlist,
    _Securitystatus,
    _Ordercancelrequest,
    _Ordercancelreplacerequest,
    _Tradingsessionstatusrequest,
    _Tradingsessionstatus,
    _Orderstatusrequest,
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
    _Xmlmessage,
    _Registrationinstructions,
    _Allocationack,
    _Registrationinstructionsresponse,
    _Ordermasscancelrequest,
    _Dontknowtrade,
    _Ordermasscancelreport,
    _Quoterequest,
    _Neworders,
    _Quote,
    _Crossordercancelreplacerequest,
    _Settlementinstructions,
    _Crossordercancelrequest,
    _Securitytyperequest,
    _Marketdatarequest,
    _Securitytypes,
    _Marketdatasnapshotfullrefresh,
    _Securitylistrequest,
    _Marketdataincrementalrefresh,
    _Securitylist,
    _Marketdatarequestreject,
    _Derivativesecuritylistrequest,
    _Quotecancel,
}
impl Field for NewSeqNo {
    fn tag(&self) -> u16 {
        return 36;
    }
}
pub struct NewSeqNo {
    value: u64,
}
impl Field for OrderID {
    fn tag(&self) -> u16 {
        return 37;
    }
}
pub struct OrderID {
    value: String,
}
impl Field for OrderQty {
    fn tag(&self) -> u16 {
        return 38;
    }
}
pub struct OrderQty {
    value: f32,
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
    _Replaced,
    _Filled,
    _Pendingcancel,
    _Stopped,
    _Rejected,
    _Suspended,
    _Pendingnew,
    _Calculated,
    _Expired,
    _Acceptedforbidding,
    _Pendingreplace,
    _Doneforday,
    _Canceled,
}
impl Field for OrdType {
    fn tag(&self) -> u16 {
        return 40;
    }
}
#[derive(Debug)]
pub enum OrdType {
    _Previouslyquoted,
    _Limit,
    _Stop,
    _Stoplimit,
    _Marketonclose,
    _Withorwithout,
    _Limitorbetter,
    _Limitwithorwithout,
    _Onbasis,
    _Onclose,
    _Market,
    _Forexc,
    _Forexf,
    _Previouslyindicated,
    _Forexg,
    _Funari,
    _Marketiftouched,
    _Marketwithleftoveraslimit,
    _Previousfundvaluationpoint,
    _Nextfundvaluationpoint,
    _Pegged,
    _Limitonclose,
    _Forexh,
}
impl Field for OrigClOrdID {
    fn tag(&self) -> u16 {
        return 41;
    }
}
pub struct OrigClOrdID {
    value: String,
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
    value: u64,
}
impl Field for Rule80A {
    fn tag(&self) -> u16 {
        return 47;
    }
}
#[derive(Debug)]
pub enum Rule80A {
    _Programordernonindexarbforothermember,
    _Shortexempttransactionb,
    _Programorderindexarbformemberfirmorg,
    _Shortexempttransactionforprincipal,
    _Shortexempttransactionf,
    _Shortexempttransactionh,
    _Individualinvestorsingleorder,
    _Programorderindexarbforindividualcustomer,
    _Programordernonindexarbforindividualcustomer,
    _Programorderindexarbforothermember,
    _Agencysingleorder,
    _Proprietarytransactionsforcompetingmarketmakerthatisaffiliatedwiththeclearingmember,
    _Principal,
    _Transactionsfortheaccountofanonmembercompetingmarketmaker,
    _Specialisttrades,
    _Transactionsfortheaccountofanunaffiliatedmemberscompetingmarketmaker,
    _Programorderindexarbforotheragency,
    _Allotherordersasagentforothermember,
    _Shortexempttransactionformembercompetingmarketmakernotaffiliatedwiththefirmclearingthetrade,
    _Programordernonindexarbforotheragency,
    _Shortexempttransactionfornonmembercompetingmarketmaker,
    _Shortexempttransactionformembercompetingmarketmakeraffiliatedwiththefirmclearingthetrade,
    _Programordernonindexarbformemberfirmorg,
}
impl Field for SecurityID {
    fn tag(&self) -> u16 {
        return 48;
    }
}
pub struct SecurityID {
    value: String,
}
impl Field for SenderCompID {
    fn tag(&self) -> u16 {
        return 49;
    }
}
pub struct SenderCompID {
    value: String,
}
impl Field for SenderSubID {
    fn tag(&self) -> u16 {
        return 50;
    }
}
pub struct SenderSubID {
    value: String,
}
impl Field for SendingTime {
    fn tag(&self) -> u16 {
        return 52;
    }
}
pub struct SendingTime {
    value: u64,
}
impl Field for Quantity {
    fn tag(&self) -> u16 {
        return 53;
    }
}
pub struct Quantity {
    value: f32,
}
impl Field for Side {
    fn tag(&self) -> u16 {
        return 54;
    }
}
#[derive(Debug)]
pub enum Side {
    _Sellshortexempt,
    _Asdefined,
    _Opposite,
    _Cross,
    _Crossshort,
    _Buy,
    _Sell,
    _Buyminus,
    _Sellplus,
    _Crossshortexempt,
    _Sellshort,
    _Undisclosed,
}
impl Field for Symbol {
    fn tag(&self) -> u16 {
        return 55;
    }
}
pub struct Symbol {
    value: String,
}
impl Field for TargetCompID {
    fn tag(&self) -> u16 {
        return 56;
    }
}
pub struct TargetCompID {
    value: String,
}
impl Field for TargetSubID {
    fn tag(&self) -> u16 {
        return 57;
    }
}
pub struct TargetSubID {
    value: String,
}
impl Field for Text {
    fn tag(&self) -> u16 {
        return 58;
    }
}
pub struct Text {
    value: String,
}
impl Field for TimeInForce {
    fn tag(&self) -> u16 {
        return 59;
    }
}
#[derive(Debug)]
pub enum TimeInForce {
    _Attheclose,
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
    _Flash,
    _Background,
    _Normal,
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
    _Tplus4,
    _Tplus1,
    _Future,
    _Tplus2,
    _Nextday,
    _Sellersoption,
    _Cash,
    _Whenandifissued,
    _Regular,
    _Tplus5,
    _Tplus3,
}
impl Field for FutSettDate {
    fn tag(&self) -> u16 {
        return 64;
    }
}
pub struct FutSettDate {
    value: u64,
}
impl Field for SymbolSfx {
    fn tag(&self) -> u16 {
        return 65;
    }
}
pub struct SymbolSfx {
    value: String,
}
impl Field for ListID {
    fn tag(&self) -> u16 {
        return 66;
    }
}
pub struct ListID {
    value: String,
}
impl Field for ListSeqNo {
    fn tag(&self) -> u16 {
        return 67;
    }
}
pub struct ListSeqNo {
    value: u16,
}
impl Field for TotNoOrders {
    fn tag(&self) -> u16 {
        return 68;
    }
}
pub struct TotNoOrders {
    value: u16,
}
impl Field for ListExecInst {
    fn tag(&self) -> u16 {
        return 69;
    }
}
pub struct ListExecInst {
    value: String,
}
impl Field for AllocID {
    fn tag(&self) -> u16 {
        return 70;
    }
}
pub struct AllocID {
    value: String,
}
impl Field for AllocTransType {
    fn tag(&self) -> u16 {
        return 71;
    }
}
#[derive(Debug)]
pub enum AllocTransType {
    _Calculatedwithoutpreliminary,
    _Calculated,
    _Preliminary,
    _Cancel,
    _Replace,
    _New,
}
impl Field for RefAllocID {
    fn tag(&self) -> u16 {
        return 72;
    }
}
pub struct RefAllocID {
    value: String,
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
    value: u64,
}
impl Field for PositionEffect {
    fn tag(&self) -> u16 {
        return 77;
    }
}
#[derive(Debug)]
pub enum PositionEffect {
    _Fifo,
    _Rolled,
    _Close,
    _Open,
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
    value: String,
}
impl Field for AllocQty {
    fn tag(&self) -> u16 {
        return 80;
    }
}
pub struct AllocQty {
    value: f32,
}
impl Field for ProcessCode {
    fn tag(&self) -> u16 {
        return 81;
    }
}
#[derive(Debug)]
pub enum ProcessCode {
    _Plansponsor,
    _Regular,
    _Softdollar,
    _Stepin,
    _Stepout,
    _Softdollarstepin,
    _Softdollarstepout,
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
    value: f32,
}
impl Field for AllocStatus {
    fn tag(&self) -> u16 {
        return 87;
    }
}
#[derive(Debug)]
pub enum AllocStatus {
    _Rejected,
    _Partialaccept,
    _Received,
    _Accepted,
}
impl Field for AllocRejCode {
    fn tag(&self) -> u16 {
        return 88;
    }
}
#[derive(Debug)]
pub enum AllocRejCode {
    _Unknownaccount,
    _Unknownlistid,
    _Unknownexecutingbrokermnemonic,
    _Unknownorderid,
    _Other,
    _Commissiondifference,
    _Incorrectquantity,
    _Incorrectaverageprice,
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
#[derive(Debug)]
pub enum PossResend {
    _No,
    _Yes,
}
impl Field for EncryptMethod {
    fn tag(&self) -> u16 {
        return 98;
    }
}
#[derive(Debug)]
pub enum EncryptMethod {
    _Des,
    _Pemdesmd5,
    _Pgpdesmd5,
    _Pkcsdes,
    _None,
    _Pkcs,
    _Pgpdes,
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
pub struct ExDestination {
    value: String,
}
impl Field for CxlRejReason {
    fn tag(&self) -> u16 {
        return 102;
    }
}
#[derive(Debug)]
pub enum CxlRejReason {
    _Unknownorder,
    _Toolatetocancel,
    _Duplicateclordidreceived,
    _Origordmodtimedidnotmatchlasttransacttimeoforder,
    _Unabletoprocessordermasscancelrequest,
    _Orderalreadyinpendingcancelorpendingreplacestatus,
    _Broker,
}
impl Field for OrdRejReason {
    fn tag(&self) -> u16 {
        return 103;
    }
}
#[derive(Debug)]
pub enum OrdRejReason {
    _Exchangeclosed,
    _Unknownsymbol,
    _Orderexceedslimit,
    _Toolatetoenter,
    _Unknownorder,
    _Duplicateofaverballycommunicatedorder,
    _Tradealongrequired,
    _Invalidinvestorid,
    _Duplicateorder,
    _Unsupportedordercharacteristic,
    _Surveillenceoption,
    _Broker,
    _Staleorder,
}
impl Field for IOIQualifier {
    fn tag(&self) -> u16 {
        return 104;
    }
}
#[derive(Debug)]
pub enum IOIQualifier {
    _Attheopen,
    _Crossingopportunity,
    _Indication,
    _Versus,
    _Throughtheday,
    _Portfolioshown,
    _Readytotrade,
    _Allornone,
    _Takingaposition,
    _Morebehind,
    _Limit,
    _Intouchwith,
    _Vwap,
    _Attheclose,
    _Marketonclose,
    _Atthemarket,
    _Atthemidpoint,
    _Preopen,
}
impl Field for Issuer {
    fn tag(&self) -> u16 {
        return 106;
    }
}
pub struct Issuer {
    value: String,
}
impl Field for SecurityDesc {
    fn tag(&self) -> u16 {
        return 107;
    }
}
pub struct SecurityDesc {
    value: String,
}
impl Field for HeartBtInt {
    fn tag(&self) -> u16 {
        return 108;
    }
}
pub struct HeartBtInt {
    value: u16,
}
impl Field for MinQty {
    fn tag(&self) -> u16 {
        return 110;
    }
}
pub struct MinQty {
    value: f32,
}
impl Field for MaxFloor {
    fn tag(&self) -> u16 {
        return 111;
    }
}
pub struct MaxFloor {
    value: f32,
}
impl Field for TestReqID {
    fn tag(&self) -> u16 {
        return 112;
    }
}
pub struct TestReqID {
    value: String,
}
impl Field for ReportToExch {
    fn tag(&self) -> u16 {
        return 113;
    }
}
#[derive(Debug)]
pub enum ReportToExch {
    _Yes,
    _No,
}
impl Field for LocateReqd {
    fn tag(&self) -> u16 {
        return 114;
    }
}
#[derive(Debug)]
pub enum LocateReqd {
    _Yes,
    _No,
}
impl Field for OnBehalfOfCompID {
    fn tag(&self) -> u16 {
        return 115;
    }
}
pub struct OnBehalfOfCompID {
    value: String,
}
impl Field for OnBehalfOfSubID {
    fn tag(&self) -> u16 {
        return 116;
    }
}
pub struct OnBehalfOfSubID {
    value: String,
}
impl Field for QuoteID {
    fn tag(&self) -> u16 {
        return 117;
    }
}
pub struct QuoteID {
    value: String,
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
    value: f32,
}
impl Field for ForexReq {
    fn tag(&self) -> u16 {
        return 121;
    }
}
#[derive(Debug)]
pub enum ForexReq {
    _Yes,
    _No,
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
    _Yes,
    _No,
}
impl Field for NoExecs {
    fn tag(&self) -> u16 {
        return 124;
    }
}
pub struct NoExecs {
    value: u16,
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
    _Wrongside,
    _Quantityexceedsorder,
    _Nomatchingorder,
    _Priceexceedslimit,
    _Other,
    _Unknownsymbol,
}
impl Field for DeliverToCompID {
    fn tag(&self) -> u16 {
        return 128;
    }
}
pub struct DeliverToCompID {
    value: String,
}
impl Field for DeliverToSubID {
    fn tag(&self) -> u16 {
        return 129;
    }
}
pub struct DeliverToSubID {
    value: String,
}
impl Field for IOINaturalFlag {
    fn tag(&self) -> u16 {
        return 130;
    }
}
#[derive(Debug)]
pub enum IOINaturalFlag {
    _Yes,
    _No,
}
impl Field for QuoteReqID {
    fn tag(&self) -> u16 {
        return 131;
    }
}
pub struct QuoteReqID {
    value: String,
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
    value: f32,
}
impl Field for OfferSize {
    fn tag(&self) -> u16 {
        return 135;
    }
}
pub struct OfferSize {
    value: f32,
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
    value: f32,
}
impl Field for MiscFeeType {
    fn tag(&self) -> u16 {
        return 139;
    }
}
#[derive(Debug)]
pub enum MiscFeeType {
    _Localcommission,
    _Exchangefees,
    _Stamp,
    _Levy,
    _Other,
    _Markup,
    _Consumptiontax,
    _Regulatory,
    _Tax,
}
impl Field for PrevClosePx {
    fn tag(&self) -> u16 {
        return 140;
    }
}
pub struct PrevClosePx {
    value: f32,
}
impl Field for ResetSeqNumFlag {
    fn tag(&self) -> u16 {
        return 141;
    }
}
#[derive(Debug)]
pub enum ResetSeqNumFlag {
    _Yes,
    _No,
}
impl Field for SenderLocationID {
    fn tag(&self) -> u16 {
        return 142;
    }
}
pub struct SenderLocationID {
    value: String,
}
impl Field for TargetLocationID {
    fn tag(&self) -> u16 {
        return 143;
    }
}
pub struct TargetLocationID {
    value: String,
}
impl Field for OnBehalfOfLocationID {
    fn tag(&self) -> u16 {
        return 144;
    }
}
pub struct OnBehalfOfLocationID {
    value: String,
}
impl Field for DeliverToLocationID {
    fn tag(&self) -> u16 {
        return 145;
    }
}
pub struct DeliverToLocationID {
    value: String,
}
impl Field for NoRelatedSym {
    fn tag(&self) -> u16 {
        return 146;
    }
}
pub struct NoRelatedSym {
    value: u16,
}
impl Field for Subject {
    fn tag(&self) -> u16 {
        return 147;
    }
}
pub struct Subject {
    value: String,
}
impl Field for Headline {
    fn tag(&self) -> u16 {
        return 148;
    }
}
pub struct Headline {
    value: String,
}
impl Field for URLLink {
    fn tag(&self) -> u16 {
        return 149;
    }
}
pub struct URLLink {
    value: String,
}
impl Field for ExecType {
    fn tag(&self) -> u16 {
        return 150;
    }
}
#[derive(Debug)]
pub enum ExecType {
    _Pendingcancel,
    _New,
    _Partialfill,
    _Fill,
    _Canceled,
    _Replace,
    _Rejected,
    _Suspended,
    _Pendingnew,
    _Calculated,
    _Expired,
    _Restated,
    _Pendingreplace,
    _Trade,
    _Tradecorrect,
    _Tradecancel,
    _Orderstatus,
    _Doneforday,
    _Stopped,
}
impl Field for LeavesQty {
    fn tag(&self) -> u16 {
        return 151;
    }
}
pub struct LeavesQty {
    value: f32,
}
impl Field for CashOrderQty {
    fn tag(&self) -> u16 {
        return 152;
    }
}
pub struct CashOrderQty {
    value: f32,
}
impl Field for AllocAvgPx {
    fn tag(&self) -> u16 {
        return 153;
    }
}
pub struct AllocAvgPx {
    value: f32,
}
impl Field for AllocNetMoney {
    fn tag(&self) -> u16 {
        return 154;
    }
}
pub struct AllocNetMoney {
    value: f32,
}
impl Field for SettlCurrFxRate {
    fn tag(&self) -> u16 {
        return 155;
    }
}
pub struct SettlCurrFxRate {
    value: f32,
}
impl Field for SettlCurrFxRateCalc {
    fn tag(&self) -> u16 {
        return 156;
    }
}
#[derive(Debug)]
pub enum SettlCurrFxRateCalc {
    _Divide,
    _Multiply,
}
impl Field for NumDaysInterest {
    fn tag(&self) -> u16 {
        return 157;
    }
}
pub struct NumDaysInterest {
    value: u16,
}
impl Field for AccruedInterestRate {
    fn tag(&self) -> u16 {
        return 158;
    }
}
pub struct AccruedInterestRate {
    value: f32,
}
impl Field for AccruedInterestAmt {
    fn tag(&self) -> u16 {
        return 159;
    }
}
pub struct AccruedInterestAmt {
    value: f32,
}
impl Field for SettlInstMode {
    fn tag(&self) -> u16 {
        return 160;
    }
}
#[derive(Debug)]
pub enum SettlInstMode {
    _Default,
    _Specificorderforasingleaccount,
    _Specificallocationaccountstanding,
    _Standinginstructionsprovided,
    _Specificallocationaccountoverriding,
}
impl Field for AllocText {
    fn tag(&self) -> u16 {
        return 161;
    }
}
pub struct AllocText {
    value: String,
}
impl Field for SettlInstID {
    fn tag(&self) -> u16 {
        return 162;
    }
}
pub struct SettlInstID {
    value: String,
}
impl Field for SettlInstTransType {
    fn tag(&self) -> u16 {
        return 163;
    }
}
#[derive(Debug)]
pub enum SettlInstTransType {
    _New,
    _Replace,
    _Cancel,
}
impl Field for EmailThreadID {
    fn tag(&self) -> u16 {
        return 164;
    }
}
pub struct EmailThreadID {
    value: String,
}
impl Field for SettlInstSource {
    fn tag(&self) -> u16 {
        return 165;
    }
}
#[derive(Debug)]
pub enum SettlInstSource {
    _Institutionsinstructions,
    _Investor,
    _Brokersinstructions,
}
impl Field for SecurityType {
    fn tag(&self) -> u16 {
        return 167;
    }
}
#[derive(Debug)]
pub enum SecurityType {
    _Commercialpaper,
    _Variableratedemandnote,
    _Plazosfijos,
    _Promissorynote,
    _Overnight,
    _Mediumtermnotes,
    _Taxexemptcommercialpaper,
    _Amendedrestated,
    _Bridgeloan,
    _Letterofcredit,
    _Swinglinefacility,
    _Debtorinpossession,
    _Defaulted,
    _Withdrawn,
    _Liquiditynote,
    _Matured,
    _Depositnotes,
    _Retired,
    _Bankersacceptance,
    _Banknotes,
    _Billofexchanges,
    _Certificateofdeposit,
    _Callloans,
    _Replaced,
    _Mandatorytender,
    _Revolvertermloan,
    _Mortgageprivateplacement,
    _Shorttermloannote,
    _Miscellaneouspassthrough,
    _Tobeannounced,
    _Otheranticipationnotesbanganetc,
    _Mortgageinterestonly,
    _Certificateofparticipation,
    _Mortgagebackedsecurities,
    _Revenuebonds,
    _Specialassessment,
    _Specialobligation,
    _Specialtax,
    _Taxanticipationnote,
    _Taxallocation,
    _Certificateofobligation,
    _Timedeposit,
    _Generalobligationbonds,
    _Wildcardentry,
    _Warrant,
    _Mutualfund,
    _Multileginstrument,
    _Taxrevenueanticipationnote,
    _Mortgageprincipalonly,
    _Repurchaseagreement,
    _Nosecuritytype,
    _Extendedcommnote,
    _Agencypools,
    _Assetbackedsecurities,
    _Corpmortgagebackedsecurities,
    _Collateralizedmortgageobligation,
    _Ioettemortgage,
    _Reverserepurchaseagreement,
    _Foreignexchangecontract,
    _Revenueanticipationnote,
    _Revolverloan,
    _Federalagencycoupon,
    _Federalagencydiscountnote,
    _Privateexportfunding,
    _Corporatebond,
    _Corporateprivateplacement,
    _Convertiblebond,
    _Dualcurrency,
    _Indexedlinked,
    _Yankeecorporatebond,
    _Commonstock,
    _Preferredstock,
    _Bradybond,
    _Ustreasurybond,
    _Intereststripfromanybondornote,
    _Treasuryinflationprotectedsecurities,
    _Principalstripofacallablebondornote,
    _Principalstripfromanoncallablebondornote,
    _Ustreasurynotebond,
    _Ustreasurybill,
    _Termloan,
    _Structurednotes,
}
impl Field for EffectiveTime {
    fn tag(&self) -> u16 {
        return 168;
    }
}
pub struct EffectiveTime {
    value: u64,
}
impl Field for StandInstDbType {
    fn tag(&self) -> u16 {
        return 169;
    }
}
#[derive(Debug)]
pub enum StandInstDbType {
    _Other,
    _Dtcsid,
    _Aglobalcustodian,
    _Thomsonalert,
}
impl Field for StandInstDbName {
    fn tag(&self) -> u16 {
        return 170;
    }
}
pub struct StandInstDbName {
    value: String,
}
impl Field for StandInstDbID {
    fn tag(&self) -> u16 {
        return 171;
    }
}
pub struct StandInstDbID {
    value: String,
}
impl Field for SettlDeliveryType {
    fn tag(&self) -> u16 {
        return 172;
    }
}
#[derive(Debug)]
pub enum SettlDeliveryType {
    _Free,
    _Versuspayment,
}
impl Field for SettlDepositoryCode {
    fn tag(&self) -> u16 {
        return 173;
    }
}
pub struct SettlDepositoryCode {
    value: String,
}
impl Field for SettlBrkrCode {
    fn tag(&self) -> u16 {
        return 174;
    }
}
pub struct SettlBrkrCode {
    value: String,
}
impl Field for SettlInstCode {
    fn tag(&self) -> u16 {
        return 175;
    }
}
pub struct SettlInstCode {
    value: String,
}
impl Field for SecuritySettlAgentName {
    fn tag(&self) -> u16 {
        return 176;
    }
}
pub struct SecuritySettlAgentName {
    value: String,
}
impl Field for SecuritySettlAgentCode {
    fn tag(&self) -> u16 {
        return 177;
    }
}
pub struct SecuritySettlAgentCode {
    value: String,
}
impl Field for SecuritySettlAgentAcctNum {
    fn tag(&self) -> u16 {
        return 178;
    }
}
pub struct SecuritySettlAgentAcctNum {
    value: String,
}
impl Field for SecuritySettlAgentAcctName {
    fn tag(&self) -> u16 {
        return 179;
    }
}
pub struct SecuritySettlAgentAcctName {
    value: String,
}
impl Field for SecuritySettlAgentContactName {
    fn tag(&self) -> u16 {
        return 180;
    }
}
pub struct SecuritySettlAgentContactName {
    value: String,
}
impl Field for SecuritySettlAgentContactPhone {
    fn tag(&self) -> u16 {
        return 181;
    }
}
pub struct SecuritySettlAgentContactPhone {
    value: String,
}
impl Field for CashSettlAgentName {
    fn tag(&self) -> u16 {
        return 182;
    }
}
pub struct CashSettlAgentName {
    value: String,
}
impl Field for CashSettlAgentCode {
    fn tag(&self) -> u16 {
        return 183;
    }
}
pub struct CashSettlAgentCode {
    value: String,
}
impl Field for CashSettlAgentAcctNum {
    fn tag(&self) -> u16 {
        return 184;
    }
}
pub struct CashSettlAgentAcctNum {
    value: String,
}
impl Field for CashSettlAgentAcctName {
    fn tag(&self) -> u16 {
        return 185;
    }
}
pub struct CashSettlAgentAcctName {
    value: String,
}
impl Field for CashSettlAgentContactName {
    fn tag(&self) -> u16 {
        return 186;
    }
}
pub struct CashSettlAgentContactName {
    value: String,
}
impl Field for CashSettlAgentContactPhone {
    fn tag(&self) -> u16 {
        return 187;
    }
}
pub struct CashSettlAgentContactPhone {
    value: String,
}
impl Field for BidSpotRate {
    fn tag(&self) -> u16 {
        return 188;
    }
}
pub struct BidSpotRate {
    value: f32,
}
impl Field for BidForwardPoints {
    fn tag(&self) -> u16 {
        return 189;
    }
}
pub struct BidForwardPoints {
    value: i8,
}
impl Field for OfferSpotRate {
    fn tag(&self) -> u16 {
        return 190;
    }
}
pub struct OfferSpotRate {
    value: f32,
}
impl Field for OfferForwardPoints {
    fn tag(&self) -> u16 {
        return 191;
    }
}
pub struct OfferForwardPoints {
    value: i8,
}
impl Field for OrderQty2 {
    fn tag(&self) -> u16 {
        return 192;
    }
}
pub struct OrderQty2 {
    value: f32,
}
impl Field for FutSettDate2 {
    fn tag(&self) -> u16 {
        return 193;
    }
}
pub struct FutSettDate2 {
    value: u64,
}
impl Field for LastSpotRate {
    fn tag(&self) -> u16 {
        return 194;
    }
}
pub struct LastSpotRate {
    value: f32,
}
impl Field for LastForwardPoints {
    fn tag(&self) -> u16 {
        return 195;
    }
}
pub struct LastForwardPoints {
    value: i8,
}
impl Field for AllocLinkID {
    fn tag(&self) -> u16 {
        return 196;
    }
}
pub struct AllocLinkID {
    value: String,
}
impl Field for AllocLinkType {
    fn tag(&self) -> u16 {
        return 197;
    }
}
#[derive(Debug)]
pub enum AllocLinkType {
    _Fxnetting,
    _Fxswap,
}
impl Field for SecondaryOrderID {
    fn tag(&self) -> u16 {
        return 198;
    }
}
pub struct SecondaryOrderID {
    value: String,
}
impl Field for NoIOIQualifiers {
    fn tag(&self) -> u16 {
        return 199;
    }
}
pub struct NoIOIQualifiers {
    value: u16,
}
impl Field for MaturityMonthYear {
    fn tag(&self) -> u16 {
        return 200;
    }
}
pub struct MaturityMonthYear {
    value: u8,
}
impl Field for StrikePrice {
    fn tag(&self) -> u16 {
        return 202;
    }
}
pub struct StrikePrice {
    value: f32,
}
impl Field for CoveredOrUncovered {
    fn tag(&self) -> u16 {
        return 203;
    }
}
#[derive(Debug)]
pub enum CoveredOrUncovered {
    _Uncovered,
    _Covered,
}
impl Field for OptAttribute {
    fn tag(&self) -> u16 {
        return 206;
    }
}
pub struct OptAttribute {
    value: char,
}
impl Field for SecurityExchange {
    fn tag(&self) -> u16 {
        return 207;
    }
}
pub struct SecurityExchange {
    value: String,
}
impl Field for NotifyBrokerOfCredit {
    fn tag(&self) -> u16 {
        return 208;
    }
}
#[derive(Debug)]
pub enum NotifyBrokerOfCredit {
    _No,
    _Yes,
}
impl Field for AllocHandlInst {
    fn tag(&self) -> u16 {
        return 209;
    }
}
#[derive(Debug)]
pub enum AllocHandlInst {
    _Forwardandmatch,
    _Forward,
    _Match,
}
impl Field for MaxShow {
    fn tag(&self) -> u16 {
        return 210;
    }
}
pub struct MaxShow {
    value: f32,
}
impl Field for PegDifference {
    fn tag(&self) -> u16 {
        return 211;
    }
}
pub struct PegDifference {
    value: i8,
}
impl Field for XmlDataLen {
    fn tag(&self) -> u16 {
        return 212;
    }
}
pub struct XmlDataLen {
    value: usize,
}
impl Field for XmlData {
    fn tag(&self) -> u16 {
        return 213;
    }
}
pub struct XmlData {
    value: [u8; 1024],
}
impl Field for SettlInstRefID {
    fn tag(&self) -> u16 {
        return 214;
    }
}
pub struct SettlInstRefID {
    value: String,
}
impl Field for NoRoutingIDs {
    fn tag(&self) -> u16 {
        return 215;
    }
}
pub struct NoRoutingIDs {
    value: u16,
}
impl Field for RoutingType {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 217;
    }
}
pub struct RoutingID {
    value: String,
}
impl Field for Spread {
    fn tag(&self) -> u16 {
        return 218;
    }
}
pub struct Spread {
    value: i8,
}
impl Field for Benchmark {
    fn tag(&self) -> u16 {
        return 219;
    }
}
#[derive(Debug)]
pub enum Benchmark {
    _Old10,
    _Curve,
    _5Yr,
    _10Yr,
    _30Yr,
    _Old30,
    _3Molibor,
    _6Molibor,
    _Old5,
}
impl Field for BenchmarkCurveCurrency {
    fn tag(&self) -> u16 {
        return 220;
    }
}
pub struct BenchmarkCurveCurrency {
    value: f32,
}
impl Field for BenchmarkCurveName {
    fn tag(&self) -> u16 {
        return 221;
    }
}
#[derive(Debug)]
pub enum BenchmarkCurveName {
    _Swap,
    _Libid,
    _Other,
    _Treasury,
    _Euribor,
    _Pfandbriefe,
    _Futureswap,
    _Muniaaa,
    _Libor,
}
impl Field for BenchmarkCurvePoint {
    fn tag(&self) -> u16 {
        return 222;
    }
}
pub struct BenchmarkCurvePoint {
    value: String,
}
impl Field for CouponRate {
    fn tag(&self) -> u16 {
        return 223;
    }
}
pub struct CouponRate {
    value: f32,
}
impl Field for CouponPaymentDate {
    fn tag(&self) -> u16 {
        return 224;
    }
}
pub struct CouponPaymentDate {
    value: String,
}
impl Field for IssueDate {
    fn tag(&self) -> u16 {
        return 225;
    }
}
pub struct IssueDate {
    value: String,
}
impl Field for RepurchaseTerm {
    fn tag(&self) -> u16 {
        return 226;
    }
}
pub struct RepurchaseTerm {
    value: u16,
}
impl Field for RepurchaseRate {
    fn tag(&self) -> u16 {
        return 227;
    }
}
pub struct RepurchaseRate {
    value: f32,
}
impl Field for Factor {
    fn tag(&self) -> u16 {
        return 228;
    }
}
pub struct Factor {
    value: f32,
}
impl Field for TradeOriginationDate {
    fn tag(&self) -> u16 {
        return 229;
    }
}
pub struct TradeOriginationDate {
    value: String,
}
impl Field for ExDate {
    fn tag(&self) -> u16 {
        return 230;
    }
}
pub struct ExDate {
    value: String,
}
impl Field for ContractMultiplier {
    fn tag(&self) -> u16 {
        return 231;
    }
}
pub struct ContractMultiplier {
    value: f32,
}
impl Field for NoStipulations {
    fn tag(&self) -> u16 {
        return 232;
    }
}
pub struct NoStipulations {
    value: u16,
}
impl Field for StipulationType {
    fn tag(&self) -> u16 {
        return 233;
    }
}
#[derive(Debug)]
pub enum StipulationType {
    _Absoluteprepaymentspeed,
    _Weightedaverageloanage,
    _Weightedaveragematurity,
    _Constantprepaymentrate,
    _Finalcprofhomeequityprepaymentcurve,
    _Weightedaveragelife,
    _Ofmanufacturedhousingprepaymentcurve,
    _Singlemonthlymortality,
    _Monthlyprepaymentrate,
    _Ofbmaprepaymentcurve,
    _Ofprospectusprepaymentcurve,
    _Constantprepaymentpenalty,
    _Lotvariance,
    _Constantprepaymentyield,
    _Weightedaveragecoupon,
    _Yearofissue,
    _Maturityyear,
    _Numberofpieces,
    _Poolsmaximum,
    _Poolspermillion,
    _Poolsperlot,
    _Poolspertrade,
    _Productionyear,
    _Tradevariance,
    _Geographics,
}
impl Field for StipulationValue {
    fn tag(&self) -> u16 {
        return 234;
    }
}
pub struct StipulationValue {
    value: String,
}
impl Field for YieldType {
    fn tag(&self) -> u16 {
        return 235;
    }
}
#[derive(Debug)]
pub enum YieldType { _Trueyieldtheyieldcalculatedwithcoupondatesmovedfromaweekendorholidaytothenextvalidsettlementdate,_Previouscloseyieldtheyieldofabondbasedontheclosingprice1Dayago,_Yieldtolongestaverage,_Yieldtolongestaveragelifetheyieldassumingonlymandatorysinksaretakenthisresultsinalowerpaydownofdebttheyieldisthencalculatedtothefinalpaymentdate,_Yieldtomaturitytheyieldofabondtoitsmaturitydate,_Marktomarketyieldanadjustmentinthevaluationofasecuritiesportfoliotoreflectthecurrentmarketvaluesoftherespectivesecuritiesintheportfolio,_Openaverageyieldtheaverageyieldoftherespectivesecuritiesintheportfolio,_Yieldtonextputtheyieldtothedateatwhichthebondholdercannextputthebondtotheissuer,_Proceedsyieldthecdequivalentyieldwhentheremainingtimetomaturityislessthantwoyears,_Semiannualyieldtheyieldofabondwhosecouponpaymentsarereinvestedsemiannually,_Yieldtoshortestaveragelifesameasavglifeabove,_Yieldtoshortestaverage,_Simpleyieldtheyieldofabondassumingnoreinvestmentofcouponpayments,_Yieldtotenderdatetheyieldonamunicipalbondtoitsmandatorytenderdate,_Yieldvalueof132Theamountthattheyieldwillchangefora132Ndchangeinprice,_Yieldtoworstconventionthelowestyieldtoallpossibleredemptiondatescenarios,_Taxequivalentyieldtheaftertaxyieldgrossedupbythemaximumfederaltaxrateof396Forcomparisontotaxableyields,_Annualyieldtheannualinterestordividendincomeaninvestmentearnsexpressedasapercentageoftheinvestmentstotalvalue,_Closingyieldmostrecentyeartheyieldofabondbasedontheclosingpriceasofthemostrecentyearsend,_Yieldtonextrefund,_Aftertaxyield,_Yieldatissue,_Yieldtoaveragelifetheyieldassumingthatallsinks,_Yieldtoaveragematuritytheyieldachievedbysubstitutingabondsaveragematurityfortheissuesfinalmaturitydate,_Bookyieldtheyieldofasecuritycalculatedbyusingitsbookvalueinsteadofthecurrentmarketpricethistermistypicallyusedintheusdomesticmarket,_Yieldtonextcalltheyieldofabondtothenextpossiblecalldate,_Yieldchangesinceclosethechangeintheyieldsincethepreviousdaysclosingyield,_Compoundyieldtheyieldofcertainjapanesebondsbasedonitspricecertainjapanesebondshaveirregularfirstorlastcouponsandtheyieldiscalculatedcompoundfortheseirregularperiods,_Currentyieldannualinterestonabonddividedbythemarketvaluetheactualincomerateofreturnasopposedtothecouponrateexpressedasapercentage,_Truegrossyieldyieldcalculatedusingthepriceincludingaccruedinterestwherecoupondatesaremovedfromholidaysandweekendstothenexttradingday,_Governmentequivalentyieldaskyieldbasedonsemiannualcouponscompoundinginallperiodsandactualactualcalendar,_Yieldwithinflationassumptionbasedonpricethereturnaninvestorwouldrequireonanormalbondthatwouldmaketherealreturnequaltothatoftheinflationindexedbondassumingaconstantinflationrate,_Inversefloaterbondyieldinversefloatersemiannualbondequivalentrate,_Closingyieldmostrecentquartertheyieldofabondbasedontheclosingpriceasofthemostrecentquartersend,_Mostrecentclosingyieldthelastavailableyieldstoredinhistorycomputedusingprice,_Closingyieldmostrecentmonththeyieldofabondbasedontheclosingpriceasofthemostrecentmonthsend,_Closingyieldtheyieldofabondbasedontheclosingprice }
impl Field for Yield {
    fn tag(&self) -> u16 {
        return 236;
    }
}
pub struct Yield {
    value: f32,
}
impl Field for TotalTakedown {
    fn tag(&self) -> u16 {
        return 237;
    }
}
pub struct TotalTakedown {
    value: f32,
}
impl Field for Concession {
    fn tag(&self) -> u16 {
        return 238;
    }
}
pub struct Concession {
    value: f32,
}
impl Field for RepoCollateralSecurityType {
    fn tag(&self) -> u16 {
        return 239;
    }
}
pub struct RepoCollateralSecurityType {
    value: String,
}
impl Field for RedemptionDate {
    fn tag(&self) -> u16 {
        return 240;
    }
}
pub struct RedemptionDate {
    value: String,
}
impl Field for UnderlyingCouponPaymentDate {
    fn tag(&self) -> u16 {
        return 241;
    }
}
pub struct UnderlyingCouponPaymentDate {
    value: String,
}
impl Field for UnderlyingIssueDate {
    fn tag(&self) -> u16 {
        return 242;
    }
}
pub struct UnderlyingIssueDate {
    value: String,
}
impl Field for UnderlyingRepoCollateralSecurityType {
    fn tag(&self) -> u16 {
        return 243;
    }
}
pub struct UnderlyingRepoCollateralSecurityType {
    value: String,
}
impl Field for UnderlyingRepurchaseTerm {
    fn tag(&self) -> u16 {
        return 244;
    }
}
pub struct UnderlyingRepurchaseTerm {
    value: u16,
}
impl Field for UnderlyingRepurchaseRate {
    fn tag(&self) -> u16 {
        return 245;
    }
}
pub struct UnderlyingRepurchaseRate {
    value: f32,
}
impl Field for UnderlyingFactor {
    fn tag(&self) -> u16 {
        return 246;
    }
}
pub struct UnderlyingFactor {
    value: f32,
}
impl Field for UnderlyingRedemptionDate {
    fn tag(&self) -> u16 {
        return 247;
    }
}
pub struct UnderlyingRedemptionDate {
    value: String,
}
impl Field for LegCouponPaymentDate {
    fn tag(&self) -> u16 {
        return 248;
    }
}
pub struct LegCouponPaymentDate {
    value: String,
}
impl Field for LegIssueDate {
    fn tag(&self) -> u16 {
        return 249;
    }
}
pub struct LegIssueDate {
    value: String,
}
impl Field for LegRepoCollateralSecurityType {
    fn tag(&self) -> u16 {
        return 250;
    }
}
pub struct LegRepoCollateralSecurityType {
    value: String,
}
impl Field for LegRepurchaseTerm {
    fn tag(&self) -> u16 {
        return 251;
    }
}
pub struct LegRepurchaseTerm {
    value: u16,
}
impl Field for LegRepurchaseRate {
    fn tag(&self) -> u16 {
        return 252;
    }
}
pub struct LegRepurchaseRate {
    value: f32,
}
impl Field for LegFactor {
    fn tag(&self) -> u16 {
        return 253;
    }
}
pub struct LegFactor {
    value: f32,
}
impl Field for LegRedemptionDate {
    fn tag(&self) -> u16 {
        return 254;
    }
}
pub struct LegRedemptionDate {
    value: String,
}
impl Field for CreditRating {
    fn tag(&self) -> u16 {
        return 255;
    }
}
pub struct CreditRating {
    value: String,
}
impl Field for UnderlyingCreditRating {
    fn tag(&self) -> u16 {
        return 256;
    }
}
pub struct UnderlyingCreditRating {
    value: String,
}
impl Field for LegCreditRating {
    fn tag(&self) -> u16 {
        return 257;
    }
}
pub struct LegCreditRating {
    value: String,
}
impl Field for TradedFlatSwitch {
    fn tag(&self) -> u16 {
        return 258;
    }
}
#[derive(Debug)]
pub enum TradedFlatSwitch {
    _No,
    _Yes,
}
impl Field for BasisFeatureDate {
    fn tag(&self) -> u16 {
        return 259;
    }
}
pub struct BasisFeatureDate {
    value: String,
}
impl Field for BasisFeaturePrice {
    fn tag(&self) -> u16 {
        return 260;
    }
}
pub struct BasisFeaturePrice {
    value: f32,
}
impl Field for MDReqID {
    fn tag(&self) -> u16 {
        return 262;
    }
}
pub struct MDReqID {
    value: String,
}
impl Field for SubscriptionRequestType {
    fn tag(&self) -> u16 {
        return 263;
    }
}
#[derive(Debug)]
pub enum SubscriptionRequestType {
    _Snapshotplusupdates,
    _Disableprevioussnapshotplusupdaterequest,
    _Snapshot,
}
impl Field for MarketDepth {
    fn tag(&self) -> u16 {
        return 264;
    }
}
pub struct MarketDepth {
    value: u16,
}
impl Field for MDUpdateType {
    fn tag(&self) -> u16 {
        return 265;
    }
}
#[derive(Debug)]
pub enum MDUpdateType {
    _Fullrefresh,
    _Incrementalrefresh,
}
impl Field for AggregatedBook {
    fn tag(&self) -> u16 {
        return 266;
    }
}
#[derive(Debug)]
pub enum AggregatedBook {
    _Yes,
    _No,
}
impl Field for NoMDEntryTypes {
    fn tag(&self) -> u16 {
        return 267;
    }
}
pub struct NoMDEntryTypes {
    value: u16,
}
impl Field for NoMDEntries {
    fn tag(&self) -> u16 {
        return 268;
    }
}
pub struct NoMDEntries {
    value: u16,
}
impl Field for MDEntryType {
    fn tag(&self) -> u16 {
        return 269;
    }
}
#[derive(Debug)]
pub enum MDEntryType {
    _Tradingsessionhighprice,
    _Offer,
    _Imbalance,
    _Tradingsessionvwapprice,
    _Tradingsessionlowprice,
    _Closingprice,
    _Openingprice,
    _Bid,
    _Trade,
    _Indexvalue,
    _Settlementprice,
}
impl Field for MDEntryPx {
    fn tag(&self) -> u16 {
        return 270;
    }
}
pub struct MDEntryPx {
    value: f32,
}
impl Field for MDEntrySize {
    fn tag(&self) -> u16 {
        return 271;
    }
}
pub struct MDEntrySize {
    value: f32,
}
impl Field for MDEntryDate {
    fn tag(&self) -> u16 {
        return 272;
    }
}
pub struct MDEntryDate {
    value: String,
}
impl Field for MDEntryTime {
    fn tag(&self) -> u16 {
        return 273;
    }
}
pub struct MDEntryTime {
    value: String,
}
impl Field for TickDirection {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 275;
    }
}
pub struct MDMkt {
    value: String,
}
impl Field for QuoteCondition {
    fn tag(&self) -> u16 {
        return 276;
    }
}
#[derive(Debug)]
pub enum QuoteCondition {
    _Locked,
    _Nonfirm,
    _Fasttrading,
    _Crossed,
    _Consolidatedbest,
    _Exchangebest,
    _Closed,
    _Open,
    _Depth,
}
impl Field for TradeCondition {
    fn tag(&self) -> u16 {
        return 277;
    }
}
#[derive(Debug)]
pub enum TradeCondition {
    _Nextdaytrade,
    _Opened,
    _Seller,
    _Averagepricetrade,
    _Sold,
    _Rule155Trade,
    _Stoppedstock,
    _Imbalancemorebuyers,
    _Imbalancemoresellers,
    _Openingprice,
    _Soldlast,
    _Cash,
    _Cashtrade,
    _Opening,
    _Intradaytradedetail,
    _Rule127Trade,
    _Nextday,
}
impl Field for MDEntryID {
    fn tag(&self) -> u16 {
        return 278;
    }
}
pub struct MDEntryID {
    value: String,
}
impl Field for MDUpdateAction {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 280;
    }
}
pub struct MDEntryRefID {
    value: String,
}
impl Field for MDReqRejReason {
    fn tag(&self) -> u16 {
        return 281;
    }
}
#[derive(Debug)]
pub enum MDReqRejReason {
    _Unsupportedaggregatedbook,
    _Duplicatemdreqid,
    _Unsupportedmdimplicitdelete,
    _Unsupportedopenclosesettleflag,
    _Unsupportedscope,
    _Unsupportedtradingsessionid,
    _Unsupportedmdentrytype,
    _Unsupportedmdupdatetype,
    _Unsupportedmarketdepth,
    _Unsupportedsubscriptionrequesttype,
    _Insufficientbandwidth,
    _Unknownsymbol,
    _Insufficientpermissions,
}
impl Field for MDEntryOriginator {
    fn tag(&self) -> u16 {
        return 282;
    }
}
pub struct MDEntryOriginator {
    value: String,
}
impl Field for LocationID {
    fn tag(&self) -> u16 {
        return 283;
    }
}
pub struct LocationID {
    value: String,
}
impl Field for DeskID {
    fn tag(&self) -> u16 {
        return 284;
    }
}
pub struct DeskID {
    value: String,
}
impl Field for DeleteReason {
    fn tag(&self) -> u16 {
        return 285;
    }
}
#[derive(Debug)]
pub enum DeleteReason {
    _Cancelation,
    _Error,
}
impl Field for OpenCloseSettleFlag {
    fn tag(&self) -> u16 {
        return 286;
    }
}
#[derive(Debug)]
pub enum OpenCloseSettleFlag {
    _Sessionopen,
    _Deliverysettlementprice,
    _Expectedprice,
    _Pricefrompreviousbusinessday,
    _Dailyopen,
}
impl Field for SellerDays {
    fn tag(&self) -> u16 {
        return 287;
    }
}
pub struct SellerDays {
    value: u16,
}
impl Field for MDEntryBuyer {
    fn tag(&self) -> u16 {
        return 288;
    }
}
pub struct MDEntryBuyer {
    value: String,
}
impl Field for MDEntrySeller {
    fn tag(&self) -> u16 {
        return 289;
    }
}
pub struct MDEntrySeller {
    value: String,
}
impl Field for MDEntryPositionNo {
    fn tag(&self) -> u16 {
        return 290;
    }
}
pub struct MDEntryPositionNo {
    value: u16,
}
impl Field for FinancialStatus {
    fn tag(&self) -> u16 {
        return 291;
    }
}
#[derive(Debug)]
pub enum FinancialStatus {
    _Bankrupt,
    _Pendingdelisting,
}
impl Field for CorporateAction {
    fn tag(&self) -> u16 {
        return 292;
    }
}
#[derive(Debug)]
pub enum CorporateAction {
    _Exdistribution,
    _Exinterest,
    _Exrights,
    _Exdividend,
    _New,
}
impl Field for DefBidSize {
    fn tag(&self) -> u16 {
        return 293;
    }
}
pub struct DefBidSize {
    value: f32,
}
impl Field for DefOfferSize {
    fn tag(&self) -> u16 {
        return 294;
    }
}
pub struct DefOfferSize {
    value: f32,
}
impl Field for NoQuoteEntries {
    fn tag(&self) -> u16 {
        return 295;
    }
}
pub struct NoQuoteEntries {
    value: u16,
}
impl Field for NoQuoteSets {
    fn tag(&self) -> u16 {
        return 296;
    }
}
pub struct NoQuoteSets {
    value: u16,
}
impl Field for QuoteStatus {
    fn tag(&self) -> u16 {
        return 297;
    }
}
#[derive(Debug)]
pub enum QuoteStatus {
    _Removedfrommarket,
    _Canceledforsymbol,
    _Pending,
    _Quotenotfound,
    _Query,
    _Expired,
    _Rejected,
    _Canceledall,
    _Canceledforunderlying,
    _Canceledforsecuritytype,
    _Accepted,
}
impl Field for QuoteCancelType {
    fn tag(&self) -> u16 {
        return 298;
    }
}
#[derive(Debug)]
pub enum QuoteCancelType {
    _Cancelallquotes,
    _Cancelforsecuritytype,
    _Cancelforsymbol,
    _Cancelforunderlyingsymbol,
}
impl Field for QuoteEntryID {
    fn tag(&self) -> u16 {
        return 299;
    }
}
pub struct QuoteEntryID {
    value: String,
}
impl Field for QuoteRejectReason {
    fn tag(&self) -> u16 {
        return 300;
    }
}
#[derive(Debug)]
pub enum QuoteRejectReason {
    _Notauthorizedtoquotesecurity,
    _Unknownsymbol,
    _Exchange,
    _Quoterequestexceedslimit,
    _Toolatetoenter,
    _Unknownquote,
    _Duplicatequote,
    _Invalidbidaskspread,
    _Invalidprice,
}
impl Field for QuoteResponseLevel {
    fn tag(&self) -> u16 {
        return 301;
    }
}
#[derive(Debug)]
pub enum QuoteResponseLevel {
    _Acknowledgeonlynegativeorerroneousquotes,
    _Noacknowledgement,
    _Acknowledgeeachquotemessages,
}
impl Field for QuoteSetID {
    fn tag(&self) -> u16 {
        return 302;
    }
}
pub struct QuoteSetID {
    value: String,
}
impl Field for QuoteRequestType {
    fn tag(&self) -> u16 {
        return 303;
    }
}
#[derive(Debug)]
pub enum QuoteRequestType {
    _Automatic,
    _Manual,
}
impl Field for TotQuoteEntries {
    fn tag(&self) -> u16 {
        return 304;
    }
}
pub struct TotQuoteEntries {
    value: u16,
}
impl Field for UnderlyingSecurityIDSource {
    fn tag(&self) -> u16 {
        return 305;
    }
}
pub struct UnderlyingSecurityIDSource {
    value: String,
}
impl Field for UnderlyingIssuer {
    fn tag(&self) -> u16 {
        return 306;
    }
}
pub struct UnderlyingIssuer {
    value: String,
}
impl Field for UnderlyingSecurityDesc {
    fn tag(&self) -> u16 {
        return 307;
    }
}
pub struct UnderlyingSecurityDesc {
    value: String,
}
impl Field for UnderlyingSecurityExchange {
    fn tag(&self) -> u16 {
        return 308;
    }
}
pub struct UnderlyingSecurityExchange {
    value: String,
}
impl Field for UnderlyingSecurityID {
    fn tag(&self) -> u16 {
        return 309;
    }
}
pub struct UnderlyingSecurityID {
    value: String,
}
impl Field for UnderlyingSecurityType {
    fn tag(&self) -> u16 {
        return 310;
    }
}
pub struct UnderlyingSecurityType {
    value: String,
}
impl Field for UnderlyingSymbol {
    fn tag(&self) -> u16 {
        return 311;
    }
}
pub struct UnderlyingSymbol {
    value: String,
}
impl Field for UnderlyingSymbolSfx {
    fn tag(&self) -> u16 {
        return 312;
    }
}
pub struct UnderlyingSymbolSfx {
    value: String,
}
impl Field for UnderlyingMaturityMonthYear {
    fn tag(&self) -> u16 {
        return 313;
    }
}
pub struct UnderlyingMaturityMonthYear {
    value: u8,
}
impl Field for UnderlyingPutOrCall {
    fn tag(&self) -> u16 {
        return 315;
    }
}
pub struct UnderlyingPutOrCall {
    value: u16,
}
impl Field for UnderlyingStrikePrice {
    fn tag(&self) -> u16 {
        return 316;
    }
}
pub struct UnderlyingStrikePrice {
    value: f32,
}
impl Field for UnderlyingOptAttribute {
    fn tag(&self) -> u16 {
        return 317;
    }
}
pub struct UnderlyingOptAttribute {
    value: char,
}
impl Field for SecurityReqID {
    fn tag(&self) -> u16 {
        return 320;
    }
}
pub struct SecurityReqID {
    value: String,
}
impl Field for SecurityRequestType {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 322;
    }
}
pub struct SecurityResponseID {
    value: String,
}
impl Field for SecurityResponseType {
    fn tag(&self) -> u16 {
        return 323;
    }
}
#[derive(Debug)]
pub enum SecurityResponseType {
    _Rejectsecurityproposal,
    _Acceptsecurityproposalasis,
    _Cannotmatchselectioncriteria,
    _Acceptsecurityproposalwithrevisionsasindicatedinthemessage,
    _Listofsecuritiesreturnedperrequest,
    _Listofsecuritytypesreturnedperrequest,
}
impl Field for SecurityStatusReqID {
    fn tag(&self) -> u16 {
        return 324;
    }
}
pub struct SecurityStatusReqID {
    value: String,
}
impl Field for UnsolicitedIndicator {
    fn tag(&self) -> u16 {
        return 325;
    }
}
#[derive(Debug)]
pub enum UnsolicitedIndicator {
    _Yes,
    _No,
}
impl Field for SecurityTradingStatus {
    fn tag(&self) -> u16 {
        return 326;
    }
}
#[derive(Debug)]
pub enum SecurityTradingStatus {
    _Unknownorinvalid,
    _Nomarketoncloseimbalance,
    _Itspreopening,
    _Newpriceindication,
    _Tradedisseminationtime,
    _Readytotrade,
    _Nottradedonthismarket,
    _Openingrotation,
    _Preopen,
    _Nomarketimbalance,
    _Notavailablefortrading,
    _Marketoncloseimbalancesell,
    _Marketoncloseimbalancebuy,
    _Marketimbalancesell,
    _Marketimbalancebuy,
    _Tradingrangeindication,
    _Priceindication,
    _Noopennoresume,
    _Resume,
    _Openingdelay,
    _Tradinghalt,
    _Fastmarket,
}
impl Field for HaltReasonChar {
    fn tag(&self) -> u16 {
        return 327;
    }
}
#[derive(Debug)]
pub enum HaltReasonChar {
    _Equipmentchangeover,
    _Additionalinformation,
    _Orderinflux,
    _Newspending,
    _Orderimbalance,
    _Newsdissemination,
}
impl Field for InViewOfCommon {
    fn tag(&self) -> u16 {
        return 328;
    }
}
#[derive(Debug)]
pub enum InViewOfCommon {
    _Yes,
    _No,
}
impl Field for DueToRelated {
    fn tag(&self) -> u16 {
        return 329;
    }
}
#[derive(Debug)]
pub enum DueToRelated {
    _Yes,
    _No,
}
impl Field for BuyVolume {
    fn tag(&self) -> u16 {
        return 330;
    }
}
pub struct BuyVolume {
    value: f32,
}
impl Field for SellVolume {
    fn tag(&self) -> u16 {
        return 331;
    }
}
pub struct SellVolume {
    value: f32,
}
impl Field for HighPx {
    fn tag(&self) -> u16 {
        return 332;
    }
}
pub struct HighPx {
    value: f32,
}
impl Field for LowPx {
    fn tag(&self) -> u16 {
        return 333;
    }
}
pub struct LowPx {
    value: f32,
}
impl Field for Adjustment {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 335;
    }
}
pub struct TradSesReqID {
    value: String,
}
impl Field for TradingSessionID {
    fn tag(&self) -> u16 {
        return 336;
    }
}
pub struct TradingSessionID {
    value: String,
}
impl Field for ContraTrader {
    fn tag(&self) -> u16 {
        return 337;
    }
}
pub struct ContraTrader {
    value: String,
}
impl Field for TradSesMethod {
    fn tag(&self) -> u16 {
        return 338;
    }
}
#[derive(Debug)]
pub enum TradSesMethod {
    _Twoparty,
    _Electronic,
    _Openoutcry,
}
impl Field for TradSesMode {
    fn tag(&self) -> u16 {
        return 339;
    }
}
#[derive(Debug)]
pub enum TradSesMode {
    _Production,
    _Testing,
    _Simulated,
}
impl Field for TradSesStatus {
    fn tag(&self) -> u16 {
        return 340;
    }
}
#[derive(Debug)]
pub enum TradSesStatus {
    _Preclose,
    _Requestrejected,
    _Preopen,
    _Closed,
    _Open,
    _Halted,
    _Unknown,
}
impl Field for TradSesStartTime {
    fn tag(&self) -> u16 {
        return 341;
    }
}
pub struct TradSesStartTime {
    value: u64,
}
impl Field for TradSesOpenTime {
    fn tag(&self) -> u16 {
        return 342;
    }
}
pub struct TradSesOpenTime {
    value: u64,
}
impl Field for TradSesPreCloseTime {
    fn tag(&self) -> u16 {
        return 343;
    }
}
pub struct TradSesPreCloseTime {
    value: u64,
}
impl Field for TradSesCloseTime {
    fn tag(&self) -> u16 {
        return 344;
    }
}
pub struct TradSesCloseTime {
    value: u64,
}
impl Field for TradSesEndTime {
    fn tag(&self) -> u16 {
        return 345;
    }
}
pub struct TradSesEndTime {
    value: u64,
}
impl Field for NumberOfOrders {
    fn tag(&self) -> u16 {
        return 346;
    }
}
pub struct NumberOfOrders {
    value: u16,
}
impl Field for MessageEncoding {
    fn tag(&self) -> u16 {
        return 347;
    }
}
#[derive(Debug)]
pub enum MessageEncoding {
    _Utf8,
    _Iso2022Jp,
    _Eucjp,
    _Shiftjis,
}
impl Field for EncodedIssuerLen {
    fn tag(&self) -> u16 {
        return 348;
    }
}
pub struct EncodedIssuerLen {
    value: usize,
}
impl Field for EncodedIssuer {
    fn tag(&self) -> u16 {
        return 349;
    }
}
pub struct EncodedIssuer {
    value: [u8; 1024],
}
impl Field for EncodedSecurityDescLen {
    fn tag(&self) -> u16 {
        return 350;
    }
}
pub struct EncodedSecurityDescLen {
    value: usize,
}
impl Field for EncodedSecurityDesc {
    fn tag(&self) -> u16 {
        return 351;
    }
}
pub struct EncodedSecurityDesc {
    value: [u8; 1024],
}
impl Field for EncodedListExecInstLen {
    fn tag(&self) -> u16 {
        return 352;
    }
}
pub struct EncodedListExecInstLen {
    value: usize,
}
impl Field for EncodedListExecInst {
    fn tag(&self) -> u16 {
        return 353;
    }
}
pub struct EncodedListExecInst {
    value: [u8; 1024],
}
impl Field for EncodedTextLen {
    fn tag(&self) -> u16 {
        return 354;
    }
}
pub struct EncodedTextLen {
    value: usize,
}
impl Field for EncodedText {
    fn tag(&self) -> u16 {
        return 355;
    }
}
pub struct EncodedText {
    value: [u8; 1024],
}
impl Field for EncodedSubjectLen {
    fn tag(&self) -> u16 {
        return 356;
    }
}
pub struct EncodedSubjectLen {
    value: usize,
}
impl Field for EncodedSubject {
    fn tag(&self) -> u16 {
        return 357;
    }
}
pub struct EncodedSubject {
    value: [u8; 1024],
}
impl Field for EncodedHeadlineLen {
    fn tag(&self) -> u16 {
        return 358;
    }
}
pub struct EncodedHeadlineLen {
    value: usize,
}
impl Field for EncodedHeadline {
    fn tag(&self) -> u16 {
        return 359;
    }
}
pub struct EncodedHeadline {
    value: [u8; 1024],
}
impl Field for EncodedAllocTextLen {
    fn tag(&self) -> u16 {
        return 360;
    }
}
pub struct EncodedAllocTextLen {
    value: usize,
}
impl Field for EncodedAllocText {
    fn tag(&self) -> u16 {
        return 361;
    }
}
pub struct EncodedAllocText {
    value: [u8; 1024],
}
impl Field for EncodedUnderlyingIssuerLen {
    fn tag(&self) -> u16 {
        return 362;
    }
}
pub struct EncodedUnderlyingIssuerLen {
    value: usize,
}
impl Field for EncodedUnderlyingIssuer {
    fn tag(&self) -> u16 {
        return 363;
    }
}
pub struct EncodedUnderlyingIssuer {
    value: [u8; 1024],
}
impl Field for EncodedUnderlyingSecurityDescLen {
    fn tag(&self) -> u16 {
        return 364;
    }
}
pub struct EncodedUnderlyingSecurityDescLen {
    value: usize,
}
impl Field for EncodedUnderlyingSecurityDesc {
    fn tag(&self) -> u16 {
        return 365;
    }
}
pub struct EncodedUnderlyingSecurityDesc {
    value: [u8; 1024],
}
impl Field for AllocPrice {
    fn tag(&self) -> u16 {
        return 366;
    }
}
pub struct AllocPrice {
    value: f32,
}
impl Field for QuoteSetValidUntilTime {
    fn tag(&self) -> u16 {
        return 367;
    }
}
pub struct QuoteSetValidUntilTime {
    value: u64,
}
impl Field for QuoteEntryRejectReason {
    fn tag(&self) -> u16 {
        return 368;
    }
}
pub struct QuoteEntryRejectReason {
    value: u16,
}
impl Field for LastMsgSeqNumProcessed {
    fn tag(&self) -> u16 {
        return 369;
    }
}
pub struct LastMsgSeqNumProcessed {
    value: u64,
}
impl Field for OnBehalfOfSendingTime {
    fn tag(&self) -> u16 {
        return 370;
    }
}
pub struct OnBehalfOfSendingTime {
    value: u64,
}
impl Field for RefTagID {
    fn tag(&self) -> u16 {
        return 371;
    }
}
pub struct RefTagID {
    value: u16,
}
impl Field for RefMsgType {
    fn tag(&self) -> u16 {
        return 372;
    }
}
pub struct RefMsgType {
    value: String,
}
impl Field for SessionRejectReason {
    fn tag(&self) -> u16 {
        return 373;
    }
}
#[derive(Debug)]
pub enum SessionRejectReason {
    _Xmlvalidationerror,
    _Nondatavalueincludesfielddelimiter,
    _Incorrectnumingroupcountforrepeatinggroup,
    _Repeatinggroupfieldsoutoforder,
    _Tagspecifiedoutofrequiredorder,
    _Invalidmsgtype,
    _Invalidtagnumber,
    _Compidproblem,
    _Signatureproblem,
    _Decryptionproblem,
    _Incorrectdataformatforvalue,
    _Valueisincorrect,
    _Tagspecifiedwithoutavalue,
    _Undefinedtag,
    _Sendingtimeaccuracyproblem,
    _Tagappearsmorethanonce,
    _Tagnotdefinedforthismessagetype,
    _Requiredtagmissing,
}
impl Field for BidRequestTransType {
    fn tag(&self) -> u16 {
        return 374;
    }
}
#[derive(Debug)]
pub enum BidRequestTransType {
    _New,
    _Cancel,
}
impl Field for ContraBroker {
    fn tag(&self) -> u16 {
        return 375;
    }
}
pub struct ContraBroker {
    value: String,
}
impl Field for ComplianceID {
    fn tag(&self) -> u16 {
        return 376;
    }
}
pub struct ComplianceID {
    value: String,
}
impl Field for SolicitedFlag {
    fn tag(&self) -> u16 {
        return 377;
    }
}
#[derive(Debug)]
pub enum SolicitedFlag {
    _No,
    _Yes,
}
impl Field for ExecRestatementReason {
    fn tag(&self) -> u16 {
        return 378;
    }
}
#[derive(Debug)]
pub enum ExecRestatementReason {
    _Cancelonsystemfailure,
    _Gtcorporateaction,
    _Market,
    _Cancelontradinghalt,
    _Partialdeclineoforderqty,
    _Brokeroption,
    _Repricingoforder,
    _Gtrenewal,
    _Verbalchange,
}
impl Field for BusinessRejectRefID {
    fn tag(&self) -> u16 {
        return 379;
    }
}
pub struct BusinessRejectRefID {
    value: String,
}
impl Field for BusinessRejectReason {
    fn tag(&self) -> u16 {
        return 380;
    }
}
#[derive(Debug)]
pub enum BusinessRejectReason {
    _Unsupportedmessagetype,
    _Delivertofirmnotavailableatthistime,
    _Applicationnotavailable,
    _Notauthorized,
    _Other,
    _Conditionallyrequiredfieldmissing,
    _Unkownid,
    _Unknownsecurity,
}
impl Field for GrossTradeAmt {
    fn tag(&self) -> u16 {
        return 381;
    }
}
pub struct GrossTradeAmt {
    value: f32,
}
impl Field for NoContraBrokers {
    fn tag(&self) -> u16 {
        return 382;
    }
}
pub struct NoContraBrokers {
    value: u16,
}
impl Field for MaxMessageSize {
    fn tag(&self) -> u16 {
        return 383;
    }
}
pub struct MaxMessageSize {
    value: usize,
}
impl Field for NoMsgTypes {
    fn tag(&self) -> u16 {
        return 384;
    }
}
pub struct NoMsgTypes {
    value: u16,
}
impl Field for MsgDirection {
    fn tag(&self) -> u16 {
        return 385;
    }
}
#[derive(Debug)]
pub enum MsgDirection {
    _Send,
    _Receive,
}
impl Field for NoTradingSessions {
    fn tag(&self) -> u16 {
        return 386;
    }
}
pub struct NoTradingSessions {
    value: u16,
}
impl Field for TotalVolumeTraded {
    fn tag(&self) -> u16 {
        return 387;
    }
}
pub struct TotalVolumeTraded {
    value: f32,
}
impl Field for DiscretionInst {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 389;
    }
}
pub struct DiscretionOffset {
    value: i8,
}
impl Field for BidID {
    fn tag(&self) -> u16 {
        return 390;
    }
}
pub struct BidID {
    value: String,
}
impl Field for ClientBidID {
    fn tag(&self) -> u16 {
        return 391;
    }
}
pub struct ClientBidID {
    value: String,
}
impl Field for ListName {
    fn tag(&self) -> u16 {
        return 392;
    }
}
pub struct ListName {
    value: String,
}
impl Field for TotalNumSecurities {
    fn tag(&self) -> u16 {
        return 393;
    }
}
pub struct TotalNumSecurities {
    value: u16,
}
impl Field for BidType {
    fn tag(&self) -> u16 {
        return 394;
    }
}
#[derive(Debug)]
pub enum BidType {
    _Nondisclosedstyle,
    _Disclosedstyle,
    _Nobiddingprocess,
}
impl Field for NumTickets {
    fn tag(&self) -> u16 {
        return 395;
    }
}
pub struct NumTickets {
    value: u16,
}
impl Field for SideValue1 {
    fn tag(&self) -> u16 {
        return 396;
    }
}
pub struct SideValue1 {
    value: f32,
}
impl Field for SideValue2 {
    fn tag(&self) -> u16 {
        return 397;
    }
}
pub struct SideValue2 {
    value: f32,
}
impl Field for NoBidDescriptors {
    fn tag(&self) -> u16 {
        return 398;
    }
}
pub struct NoBidDescriptors {
    value: u16,
}
impl Field for BidDescriptorType {
    fn tag(&self) -> u16 {
        return 399;
    }
}
#[derive(Debug)]
pub enum BidDescriptorType {
    _Index,
    _Country,
    _Sector,
}
impl Field for BidDescriptor {
    fn tag(&self) -> u16 {
        return 400;
    }
}
pub struct BidDescriptor {
    value: String,
}
impl Field for SideValueInd {
    fn tag(&self) -> u16 {
        return 401;
    }
}
#[derive(Debug)]
pub enum SideValueInd {
    _Sidevalue1,
    _Sidevalue2,
}
impl Field for LiquidityPctLow {
    fn tag(&self) -> u16 {
        return 402;
    }
}
pub struct LiquidityPctLow {
    value: f32,
}
impl Field for LiquidityPctHigh {
    fn tag(&self) -> u16 {
        return 403;
    }
}
pub struct LiquidityPctHigh {
    value: f32,
}
impl Field for LiquidityValue {
    fn tag(&self) -> u16 {
        return 404;
    }
}
pub struct LiquidityValue {
    value: f32,
}
impl Field for EFPTrackingError {
    fn tag(&self) -> u16 {
        return 405;
    }
}
pub struct EFPTrackingError {
    value: f32,
}
impl Field for FairValue {
    fn tag(&self) -> u16 {
        return 406;
    }
}
pub struct FairValue {
    value: f32,
}
impl Field for OutsideIndexPct {
    fn tag(&self) -> u16 {
        return 407;
    }
}
pub struct OutsideIndexPct {
    value: f32,
}
impl Field for ValueOfFutures {
    fn tag(&self) -> u16 {
        return 408;
    }
}
pub struct ValueOfFutures {
    value: f32,
}
impl Field for LiquidityIndType {
    fn tag(&self) -> u16 {
        return 409;
    }
}
#[derive(Debug)]
pub enum LiquidityIndType {
    _Normalmarketsize,
    _Other,
    _20Daymovingaverage,
    _5Daymovingaverage,
}
impl Field for WtAverageLiquidity {
    fn tag(&self) -> u16 {
        return 410;
    }
}
pub struct WtAverageLiquidity {
    value: f32,
}
impl Field for ExchangeForPhysical {
    fn tag(&self) -> u16 {
        return 411;
    }
}
#[derive(Debug)]
pub enum ExchangeForPhysical {
    _No,
    _Yes,
}
impl Field for OutMainCntryUIndex {
    fn tag(&self) -> u16 {
        return 412;
    }
}
pub struct OutMainCntryUIndex {
    value: f32,
}
impl Field for CrossPercent {
    fn tag(&self) -> u16 {
        return 413;
    }
}
pub struct CrossPercent {
    value: f32,
}
impl Field for ProgRptReqs {
    fn tag(&self) -> u16 {
        return 414;
    }
}
#[derive(Debug)]
pub enum ProgRptReqs {
    _Realtimeexecutionreports,
    _Sellsideperiodicallysendsstatususingliststatusperiodoptionallyspecifiedinprogressperiod,
    _Buysideexplicitlyrequestsstatususingstatusrequest,
}
impl Field for ProgPeriodInterval {
    fn tag(&self) -> u16 {
        return 415;
    }
}
pub struct ProgPeriodInterval {
    value: u16,
}
impl Field for IncTaxInd {
    fn tag(&self) -> u16 {
        return 416;
    }
}
#[derive(Debug)]
pub enum IncTaxInd {
    _Gross,
    _Net,
}
impl Field for NumBidders {
    fn tag(&self) -> u16 {
        return 417;
    }
}
pub struct NumBidders {
    value: u16,
}
impl Field for TradeType {
    fn tag(&self) -> u16 {
        return 418;
    }
}
#[derive(Debug)]
pub enum TradeType {
    _Vwapguarantee,
    _Agency,
    _Guaranteedclose,
    _Risktrade,
}
impl Field for BasisPxType {
    fn tag(&self) -> u16 {
        return 419;
    }
}
#[derive(Debug)]
pub enum BasisPxType {
    _Vwapthroughanafternoonsession,
    _Open,
    _Others,
    _Strike,
    _Vwapthroughanafternoonsessionexceptyori,
    _Vwapthroughadayexceptyori,
    _Vwapthroughamorningsession,
    _Vwapthroughaday,
    _Sq,
    _Currentprice,
    _Closingprice,
    _Closingpriceatmorningsession,
    _Vwapthroughamorningsessionexceptyori,
}
impl Field for NoBidComponents {
    fn tag(&self) -> u16 {
        return 420;
    }
}
pub struct NoBidComponents {
    value: u16,
}
impl Field for Country {
    fn tag(&self) -> u16 {
        return 421;
    }
}
pub struct Country {
    value: String,
}
impl Field for TotNoStrikes {
    fn tag(&self) -> u16 {
        return 422;
    }
}
pub struct TotNoStrikes {
    value: u16,
}
impl Field for PriceType {
    fn tag(&self) -> u16 {
        return 423;
    }
}
#[derive(Debug)]
pub enum PriceType {
    _Fixedamount,
    _Percentage,
    _Discount,
    _Basispointsrelativetobenchmark,
    _Tedprice,
    _Tedyield,
    _Premium,
    _Pershare,
}
impl Field for DayOrderQty {
    fn tag(&self) -> u16 {
        return 424;
    }
}
pub struct DayOrderQty {
    value: f32,
}
impl Field for DayCumQty {
    fn tag(&self) -> u16 {
        return 425;
    }
}
pub struct DayCumQty {
    value: f32,
}
impl Field for DayAvgPx {
    fn tag(&self) -> u16 {
        return 426;
    }
}
pub struct DayAvgPx {
    value: f32,
}
impl Field for GTBookingInst {
    fn tag(&self) -> u16 {
        return 427;
    }
}
#[derive(Debug)]
pub enum GTBookingInst {
    _Bookoutalltradesondayofexecution,
    _Accumulateuntilverballynotifiedotherwise,
    _Accumulateexecutionsuntilorderisfilledorexpires,
}
impl Field for NoStrikes {
    fn tag(&self) -> u16 {
        return 428;
    }
}
pub struct NoStrikes {
    value: u16,
}
impl Field for ListStatusType {
    fn tag(&self) -> u16 {
        return 429;
    }
}
#[derive(Debug)]
pub enum ListStatusType {
    _Alert,
    _Execstarted,
    _Timed,
    _Response,
    _Ack,
    _Alldone,
}
impl Field for NetGrossInd {
    fn tag(&self) -> u16 {
        return 430;
    }
}
#[derive(Debug)]
pub enum NetGrossInd {
    _Net,
    _Gross,
}
impl Field for ListOrderStatus {
    fn tag(&self) -> u16 {
        return 431;
    }
}
#[derive(Debug)]
pub enum ListOrderStatus {
    _Canceling,
    _Executing,
    _Reject,
    _Alldone,
    _Alert,
    _Receivedforexecution,
    _Inbiddingprocess,
}
impl Field for ExpireDate {
    fn tag(&self) -> u16 {
        return 432;
    }
}
pub struct ExpireDate {
    value: u64,
}
impl Field for ListExecInstType {
    fn tag(&self) -> u16 {
        return 433;
    }
}
#[derive(Debug)]
pub enum ListExecInstType {
    _Exchangeswitchcivorderbuydrivencashwithdraw,
    _Exchangeswitchcivorderbuydrivencashtopup,
    _Waitforexecuteinstruction,
    _Immediate,
    _Exchangeswitchcivorderselldriven,
}
impl Field for CxlRejResponseTo {
    fn tag(&self) -> u16 {
        return 434;
    }
}
#[derive(Debug)]
pub enum CxlRejResponseTo {
    _Ordercancelreplacerequest,
    _Ordercancelrequest,
}
impl Field for UnderlyingCouponRate {
    fn tag(&self) -> u16 {
        return 435;
    }
}
pub struct UnderlyingCouponRate {
    value: f32,
}
impl Field for UnderlyingContractMultiplier {
    fn tag(&self) -> u16 {
        return 436;
    }
}
pub struct UnderlyingContractMultiplier {
    value: f32,
}
impl Field for ContraTradeQty {
    fn tag(&self) -> u16 {
        return 437;
    }
}
pub struct ContraTradeQty {
    value: f32,
}
impl Field for ContraTradeTime {
    fn tag(&self) -> u16 {
        return 438;
    }
}
pub struct ContraTradeTime {
    value: u64,
}
impl Field for LiquidityNumSecurities {
    fn tag(&self) -> u16 {
        return 441;
    }
}
pub struct LiquidityNumSecurities {
    value: u16,
}
impl Field for MultiLegReportingType {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 443;
    }
}
pub struct StrikeTime {
    value: u64,
}
impl Field for ListStatusText {
    fn tag(&self) -> u16 {
        return 444;
    }
}
pub struct ListStatusText {
    value: String,
}
impl Field for EncodedListStatusTextLen {
    fn tag(&self) -> u16 {
        return 445;
    }
}
pub struct EncodedListStatusTextLen {
    value: usize,
}
impl Field for EncodedListStatusText {
    fn tag(&self) -> u16 {
        return 446;
    }
}
pub struct EncodedListStatusText {
    value: [u8; 1024],
}
impl Field for PartyIDSource {
    fn tag(&self) -> u16 {
        return 447;
    }
}
#[derive(Debug)]
pub enum PartyIDSource {
    _Chinesebshare,
    _Usemployeridentificationnumber,
    _Australiantaxfilenumber,
    _Australianbusinessnumber,
    _Isocountrycode,
    _Bic,
    _Ussocialsecuritynumber,
    _Proprietarycustomcode,
    _Settlemententitylocation,
    _Koreaninvestorid,
    _Taiwanesequalifiedforeigninvestoridqfii,
    _Taiwanesetradingaccount,
    _Malaysiancentraldepository,
    _Uknationalinsuranceorpensionnumber,
    _Generallyacceptedmarketparticipantidentifier,
}
impl Field for PartyID {
    fn tag(&self) -> u16 {
        return 448;
    }
}
pub struct PartyID {
    value: String,
}
impl Field for TotalVolumeTradedDate {
    fn tag(&self) -> u16 {
        return 449;
    }
}
pub struct TotalVolumeTradedDate {
    value: String,
}
impl Field for TotalVolumeTradedTime {
    fn tag(&self) -> u16 {
        return 450;
    }
}
pub struct TotalVolumeTradedTime {
    value: String,
}
impl Field for NetChgPrevDay {
    fn tag(&self) -> u16 {
        return 451;
    }
}
pub struct NetChgPrevDay {
    value: i8,
}
impl Field for PartyRole {
    fn tag(&self) -> u16 {
        return 452;
    }
}
#[derive(Debug)]
pub enum PartyRole {
    _Correspondantclearingfirm,
    _Clientid,
    _Underlyingcontrafirm,
    _Sponsoringfirm,
    _Contraclearingfirm,
    _Contrafirm,
    _Executingsystem,
    _Enteringfirm,
    _Executingfirm,
    _Brokerofcredit,
    _Investorid,
    _Introducingfirm,
    _Giveupclearingfirm,
    _Locatelendingfirm,
    _Fundmanagerclientid,
    _Settlementlocation,
    _Orderoriginationtrader,
    _Executingtrader,
    _Orderoriginationfirm,
    _Clearingfirm,
}
impl Field for NoPartyIDs {
    fn tag(&self) -> u16 {
        return 453;
    }
}
pub struct NoPartyIDs {
    value: u16,
}
impl Field for NoSecurityAltID {
    fn tag(&self) -> u16 {
        return 454;
    }
}
pub struct NoSecurityAltID {
    value: u16,
}
impl Field for SecurityAltID {
    fn tag(&self) -> u16 {
        return 455;
    }
}
pub struct SecurityAltID {
    value: String,
}
impl Field for SecurityAltIDSource {
    fn tag(&self) -> u16 {
        return 456;
    }
}
pub struct SecurityAltIDSource {
    value: String,
}
impl Field for NoUnderlyingSecurityAltID {
    fn tag(&self) -> u16 {
        return 457;
    }
}
pub struct NoUnderlyingSecurityAltID {
    value: u16,
}
impl Field for UnderlyingSecurityAltID {
    fn tag(&self) -> u16 {
        return 458;
    }
}
pub struct UnderlyingSecurityAltID {
    value: String,
}
impl Field for UnderlyingSecurityAltIDSource {
    fn tag(&self) -> u16 {
        return 459;
    }
}
pub struct UnderlyingSecurityAltIDSource {
    value: String,
}
impl Field for Product {
    fn tag(&self) -> u16 {
        return 460;
    }
}
#[derive(Debug)]
pub enum Product {
    _Loan,
    _Other,
    _Municipal,
    _Agency,
    _Corporate,
    _Currency,
    _Commodity,
    _Government,
    _Mortgage,
    _Index,
    _Moneymarket,
    _Equity,
}
impl Field for CFICode {
    fn tag(&self) -> u16 {
        return 461;
    }
}
pub struct CFICode {
    value: String,
}
impl Field for UnderlyingProduct {
    fn tag(&self) -> u16 {
        return 462;
    }
}
pub struct UnderlyingProduct {
    value: u16,
}
impl Field for UnderlyingCFICode {
    fn tag(&self) -> u16 {
        return 463;
    }
}
pub struct UnderlyingCFICode {
    value: String,
}
impl Field for TestMessageIndicator {
    fn tag(&self) -> u16 {
        return 464;
    }
}
#[derive(Debug)]
pub enum TestMessageIndicator {
    _Yes,
    _No,
}
impl Field for QuantityType {
    fn tag(&self) -> u16 {
        return 465;
    }
}
#[derive(Debug)]
pub enum QuantityType {
    _Contracts,
    _Other,
    _Currency,
    _Originalface,
    _Currentface,
    _Bonds,
    _Shares,
    _Par,
}
impl Field for BookingRefID {
    fn tag(&self) -> u16 {
        return 466;
    }
}
pub struct BookingRefID {
    value: String,
}
impl Field for IndividualAllocID {
    fn tag(&self) -> u16 {
        return 467;
    }
}
pub struct IndividualAllocID {
    value: String,
}
impl Field for RoundingDirection {
    fn tag(&self) -> u16 {
        return 468;
    }
}
#[derive(Debug)]
pub enum RoundingDirection {
    _Roundtonearest,
    _Rounddown,
    _Roundup,
}
impl Field for RoundingModulus {
    fn tag(&self) -> u16 {
        return 469;
    }
}
pub struct RoundingModulus {
    value: f32,
}
impl Field for CountryOfIssue {
    fn tag(&self) -> u16 {
        return 470;
    }
}
pub struct CountryOfIssue {
    value: String,
}
impl Field for StateOrProvinceOfIssue {
    fn tag(&self) -> u16 {
        return 471;
    }
}
pub struct StateOrProvinceOfIssue {
    value: String,
}
impl Field for LocaleOfIssue {
    fn tag(&self) -> u16 {
        return 472;
    }
}
pub struct LocaleOfIssue {
    value: String,
}
impl Field for NoRegistDtls {
    fn tag(&self) -> u16 {
        return 473;
    }
}
pub struct NoRegistDtls {
    value: u16,
}
impl Field for MailingDtls {
    fn tag(&self) -> u16 {
        return 474;
    }
}
pub struct MailingDtls {
    value: String,
}
impl Field for InvestorCountryOfResidence {
    fn tag(&self) -> u16 {
        return 475;
    }
}
pub struct InvestorCountryOfResidence {
    value: String,
}
impl Field for PaymentRef {
    fn tag(&self) -> u16 {
        return 476;
    }
}
pub struct PaymentRef {
    value: String,
}
impl Field for DistribPaymentMethod {
    fn tag(&self) -> u16 {
        return 477;
    }
}
pub struct DistribPaymentMethod {
    value: u16,
}
impl Field for CashDistribCurr {
    fn tag(&self) -> u16 {
        return 478;
    }
}
pub struct CashDistribCurr {
    value: f32,
}
impl Field for CommCurrency {
    fn tag(&self) -> u16 {
        return 479;
    }
}
pub struct CommCurrency {
    value: f32,
}
impl Field for CancellationRights {
    fn tag(&self) -> u16 {
        return 480;
    }
}
#[derive(Debug)]
pub enum CancellationRights {
    _Nowaiveragreement,
    _Noexecutiononly,
    _Yes,
    _Noinstitutional,
}
impl Field for MoneyLaunderingStatus {
    fn tag(&self) -> u16 {
        return 481;
    }
}
#[derive(Debug)]
pub enum MoneyLaunderingStatus {
    _Exemptauthorisedcreditorfinancialinstitution,
    _Exemptclientmoneytypeexemption,
    _Exemptbelowthelimit,
    _Passed,
    _Notchecked,
}
impl Field for MailingInst {
    fn tag(&self) -> u16 {
        return 482;
    }
}
pub struct MailingInst {
    value: String,
}
impl Field for TransBkdTime {
    fn tag(&self) -> u16 {
        return 483;
    }
}
pub struct TransBkdTime {
    value: u64,
}
impl Field for ExecPriceType {
    fn tag(&self) -> u16 {
        return 484;
    }
}
#[derive(Debug)]
pub enum ExecPriceType {
    _Singleprice,
    _Offerpriceminusadjustmentamount,
    _Offerpriceminusadjustment,
    _Offerprice,
    _Creationpriceplusadjustmentamount,
    _Creationpriceplusadjustment,
    _Creationprice,
    _Bidprice,
}
impl Field for ExecPriceAdjustment {
    fn tag(&self) -> u16 {
        return 485;
    }
}
pub struct ExecPriceAdjustment {
    value: f32,
}
impl Field for DateOfBirth {
    fn tag(&self) -> u16 {
        return 486;
    }
}
pub struct DateOfBirth {
    value: u64,
}
impl Field for TradeReportTransType {
    fn tag(&self) -> u16 {
        return 487;
    }
}
#[derive(Debug)]
pub enum TradeReportTransType {
    _New,
    _Replace,
    _Cancel,
}
impl Field for CardHolderName {
    fn tag(&self) -> u16 {
        return 488;
    }
}
pub struct CardHolderName {
    value: String,
}
impl Field for CardNumber {
    fn tag(&self) -> u16 {
        return 489;
    }
}
pub struct CardNumber {
    value: String,
}
impl Field for CardExpDate {
    fn tag(&self) -> u16 {
        return 490;
    }
}
pub struct CardExpDate {
    value: u64,
}
impl Field for CardIssNo {
    fn tag(&self) -> u16 {
        return 491;
    }
}
pub struct CardIssNo {
    value: String,
}
impl Field for PaymentMethod {
    fn tag(&self) -> u16 {
        return 492;
    }
}
#[derive(Debug)]
pub enum PaymentMethod {
    _Bpay,
    _Achcredit,
    _Achdebit,
    _Creditcard,
    _Directcredit,
    _Directdebit,
    _Debitcard,
    _Fedwire,
    _Highvalueclearingsystem,
    _Euroclear,
    _Telegraphictransfer,
    _Clearstream,
    _Crest,
    _Nscc,
    _Cheque,
}
impl Field for RegistAcctType {
    fn tag(&self) -> u16 {
        return 493;
    }
}
pub struct RegistAcctType {
    value: String,
}
impl Field for Designation {
    fn tag(&self) -> u16 {
        return 494;
    }
}
pub struct Designation {
    value: String,
}
impl Field for TaxAdvantageType {
    fn tag(&self) -> u16 {
        return 495;
    }
}
#[derive(Debug)]
pub enum TaxAdvantageType {
    _Profitsharingplan,
    _Employer,
    _Employercurrentyear,
    _Nonfundprototypeira,
    _Nonfundqualifiedplan,
    _Definedcontributionplan,
    _Employeecurrentyear,
    _Individualretirementaccountrollover,
    _Miniinsuranceisa,
    _Individualretirementaccount,
    _Employee,
    _Assettransfer,
    _Selfdirectedira,
    _Currentyearpayment,
    _401K,
    _Ministocksandsharesisa,
    _Minicashisa,
    _Tessa,
    _Maxiisa,
    _Nonenotapplicable,
    _Prioryearpayment,
    _457,
    _Rothira24,
    _Rothira25,
    _Rothconversionira26,
    _Rothconversionira27,
    _Educationira28,
    _Educationira29,
    _Keogh,
    _403,
}
impl Field for RegistRejReasonText {
    fn tag(&self) -> u16 {
        return 496;
    }
}
pub struct RegistRejReasonText {
    value: String,
}
impl Field for FundRenewWaiv {
    fn tag(&self) -> u16 {
        return 497;
    }
}
#[derive(Debug)]
pub enum FundRenewWaiv {
    _No,
    _Yes,
}
impl Field for CashDistribAgentName {
    fn tag(&self) -> u16 {
        return 498;
    }
}
pub struct CashDistribAgentName {
    value: String,
}
impl Field for CashDistribAgentCode {
    fn tag(&self) -> u16 {
        return 499;
    }
}
pub struct CashDistribAgentCode {
    value: String,
}
impl Field for CashDistribAgentAcctNumber {
    fn tag(&self) -> u16 {
        return 500;
    }
}
pub struct CashDistribAgentAcctNumber {
    value: String,
}
impl Field for CashDistribPayRef {
    fn tag(&self) -> u16 {
        return 501;
    }
}
pub struct CashDistribPayRef {
    value: String,
}
impl Field for CardStartDate {
    fn tag(&self) -> u16 {
        return 503;
    }
}
pub struct CardStartDate {
    value: u64,
}
impl Field for PaymentDate {
    fn tag(&self) -> u16 {
        return 504;
    }
}
pub struct PaymentDate {
    value: u64,
}
impl Field for PaymentRemitterID {
    fn tag(&self) -> u16 {
        return 505;
    }
}
pub struct PaymentRemitterID {
    value: String,
}
impl Field for RegistStatus {
    fn tag(&self) -> u16 {
        return 506;
    }
}
#[derive(Debug)]
pub enum RegistStatus {
    _Accept,
    _Reminder,
    _Reject,
    _Held,
}
impl Field for RegistRejReasonCode {
    fn tag(&self) -> u16 {
        return 507;
    }
}
#[derive(Debug)]
pub enum RegistRejReasonCode {
    _Invalidunacceptablenodistribinstns,
    _Invalidunacceptablecashdistribagentcode,
    _Invalidunacceptablecashdistribagentacctname,
    _Invalidunacceptablenoregdetls,
    _Invalidunacceptabledistribpaymentmethod,
    _Invalidunacceptabledistribpercentage,
    _Invalidunacceptableownershiptype,
    _Invalidunacceptabletaxexempttype,
    _Invalidunacceptableinvestorcountryofresidence,
    _Invalidunacceptabledateofbirth,
    _Invalidunacceptableinvestoridsource,
    _Invalidunacceptableinvestorid,
    _Invalidunacceptablemailinginst,
    _Invalidunacceptablemailingdtls,
    _Invalidunacceptableregseqno,
    _Invalidunacceptableaccounttype,
    _Invalidunacceptablecashdistribagentacctnum,
    _Invalidunacceptableregdtls,
}
impl Field for RegistRefID {
    fn tag(&self) -> u16 {
        return 508;
    }
}
pub struct RegistRefID {
    value: String,
}
impl Field for RegistDetls {
    fn tag(&self) -> u16 {
        return 509;
    }
}
pub struct RegistDetls {
    value: String,
}
impl Field for NoDistribInsts {
    fn tag(&self) -> u16 {
        return 510;
    }
}
pub struct NoDistribInsts {
    value: u16,
}
impl Field for RegistEmail {
    fn tag(&self) -> u16 {
        return 511;
    }
}
pub struct RegistEmail {
    value: String,
}
impl Field for DistribPercentage {
    fn tag(&self) -> u16 {
        return 512;
    }
}
pub struct DistribPercentage {
    value: f32,
}
impl Field for RegistID {
    fn tag(&self) -> u16 {
        return 513;
    }
}
pub struct RegistID {
    value: String,
}
impl Field for RegistTransType {
    fn tag(&self) -> u16 {
        return 514;
    }
}
#[derive(Debug)]
pub enum RegistTransType {
    _Cancel,
    _New,
    _Replace,
}
impl Field for ExecValuationPoint {
    fn tag(&self) -> u16 {
        return 515;
    }
}
pub struct ExecValuationPoint {
    value: u64,
}
impl Field for OrderPercent {
    fn tag(&self) -> u16 {
        return 516;
    }
}
pub struct OrderPercent {
    value: f32,
}
impl Field for OwnershipType {
    fn tag(&self) -> u16 {
        return 517;
    }
}
pub struct OwnershipType {
    value: char,
}
impl Field for NoContAmts {
    fn tag(&self) -> u16 {
        return 518;
    }
}
pub struct NoContAmts {
    value: u16,
}
impl Field for ContAmtType {
    fn tag(&self) -> u16 {
        return 519;
    }
}
#[derive(Debug)]
pub enum ContAmtType {
    _Netsettlementamount,
    _Commissionamount,
    _Commission,
    _Initialchargeamount,
    _Initialcharge,
    _Discountamount,
    _Discount,
    _Dilutionlevyamount,
    _Dilutionlevy,
    _Exitchargeamount,
    _Exitcharge,
    _Fundbasedrenewalcommission,
    _Projectedfundvalue,
    _Fundbasedrenewalcommissionamount14,
    _Fundbasedrenewalcommissionamount13,
}
impl Field for ContAmtValue {
    fn tag(&self) -> u16 {
        return 520;
    }
}
pub struct ContAmtValue {
    value: f32,
}
impl Field for ContAmtCurr {
    fn tag(&self) -> u16 {
        return 521;
    }
}
pub struct ContAmtCurr {
    value: f32,
}
impl Field for OwnerType {
    fn tag(&self) -> u16 {
        return 522;
    }
}
#[derive(Debug)]
pub enum OwnerType {
    _Companytrustee,
    _Nominee,
    _Corporatebody,
    _Nonprofitorganization,
    _Networkingsubaccount,
    _Fiduciaries,
    _Trusts,
    _Pensionplan,
    _Individualtrustee,
    _Publiccompany,
    _Privatecompany,
    _Individualinvestor,
    _Custodianundergiftstominorsact,
}
impl Field for PartySubID {
    fn tag(&self) -> u16 {
        return 523;
    }
}
pub struct PartySubID {
    value: String,
}
impl Field for NestedPartyID {
    fn tag(&self) -> u16 {
        return 524;
    }
}
pub struct NestedPartyID {
    value: String,
}
impl Field for NestedPartyIDSource {
    fn tag(&self) -> u16 {
        return 525;
    }
}
pub struct NestedPartyIDSource {
    value: char,
}
impl Field for SecondaryClOrdID {
    fn tag(&self) -> u16 {
        return 526;
    }
}
pub struct SecondaryClOrdID {
    value: String,
}
impl Field for SecondaryExecID {
    fn tag(&self) -> u16 {
        return 527;
    }
}
pub struct SecondaryExecID {
    value: String,
}
impl Field for OrderCapacity {
    fn tag(&self) -> u16 {
        return 528;
    }
}
#[derive(Debug)]
pub enum OrderCapacity {
    _Risklessprincipal,
    _Individual,
    _Principal,
    _Agentforothermember,
    _Agency,
    _Proprietary,
}
impl Field for OrderRestrictions {
    fn tag(&self) -> u16 {
        return 529;
    }
}
#[derive(Debug)]
pub enum OrderRestrictions {
    _Foreignentity,
    _Risklessarbitrage,
    _Programtrade,
    _Externalmarketparticipant,
    _Actingasmarketmakerorspecialistintheunderlyingsecurityofaderivativesecurity,
    _Actingasmarketmakerorspecialistinthesecurity,
    _Nonindexarbitrage,
    _Indexarbitrage,
    _Competingmarketmaker,
    _Externalinterconnectedmarketlinkage,
}
impl Field for MassCancelRequestType {
    fn tag(&self) -> u16 {
        return 530;
    }
}
#[derive(Debug)]
pub enum MassCancelRequestType {
    _Cancelordersforasecurity,
    _Cancelallorders,
    _Cancelordersforatradingsession,
    _Cancelordersforasecuritytype,
    _Cancelordersforacficode,
    _Cancelordersforanunderlyingsecurity,
    _Cancelordersforaproduct,
}
impl Field for MassCancelResponse {
    fn tag(&self) -> u16 {
        return 531;
    }
}
#[derive(Debug)]
pub enum MassCancelResponse {
    _Cancelordersforatradingsession,
    _Cancelrequestrejected,
    _Cancelallorders,
    _Cancelordersforaproduct,
    _Cancelordersforasecuritytype,
    _Cancelordersforacficode,
    _Cancelordersforasecurity,
    _Cancelordersforanunderlyingsecurity,
}
impl Field for MassCancelRejectReason {
    fn tag(&self) -> u16 {
        return 532;
    }
}
#[derive(Debug)]
pub enum MassCancelRejectReason {
    _Invalidorunknownunderlying,
    _Invalidorunknowntradingsession,
    _Invalidorunknownsecuritytype,
    _Invalidorunknownproduct,
    _Invalidorunknownsecurity,
    _Masscancelnotsupported,
    _Invalidorunknowncficode,
}
impl Field for TotalAffectedOrders {
    fn tag(&self) -> u16 {
        return 533;
    }
}
pub struct TotalAffectedOrders {
    value: u16,
}
impl Field for NoAffectedOrders {
    fn tag(&self) -> u16 {
        return 534;
    }
}
pub struct NoAffectedOrders {
    value: u16,
}
impl Field for AffectedOrderID {
    fn tag(&self) -> u16 {
        return 535;
    }
}
pub struct AffectedOrderID {
    value: String,
}
impl Field for AffectedSecondaryOrderID {
    fn tag(&self) -> u16 {
        return 536;
    }
}
pub struct AffectedSecondaryOrderID {
    value: String,
}
impl Field for QuoteType {
    fn tag(&self) -> u16 {
        return 537;
    }
}
#[derive(Debug)]
pub enum QuoteType {
    _Indicative,
    _Tradeable,
    _Restrictedtradeable,
}
impl Field for NestedPartyRole {
    fn tag(&self) -> u16 {
        return 538;
    }
}
pub struct NestedPartyRole {
    value: u16,
}
impl Field for NoNestedPartyIDs {
    fn tag(&self) -> u16 {
        return 539;
    }
}
pub struct NoNestedPartyIDs {
    value: u16,
}
impl Field for TotalAccruedInterestAmt {
    fn tag(&self) -> u16 {
        return 540;
    }
}
pub struct TotalAccruedInterestAmt {
    value: f32,
}
impl Field for MaturityDate {
    fn tag(&self) -> u16 {
        return 541;
    }
}
pub struct MaturityDate {
    value: u64,
}
impl Field for UnderlyingMaturityDate {
    fn tag(&self) -> u16 {
        return 542;
    }
}
pub struct UnderlyingMaturityDate {
    value: u64,
}
impl Field for InstrRegistry {
    fn tag(&self) -> u16 {
        return 543;
    }
}
pub struct InstrRegistry {
    value: String,
}
impl Field for CashMargin {
    fn tag(&self) -> u16 {
        return 544;
    }
}
#[derive(Debug)]
pub enum CashMargin {
    _Marginopen,
    _Marginclose,
    _Cash,
}
impl Field for NestedPartySubID {
    fn tag(&self) -> u16 {
        return 545;
    }
}
pub struct NestedPartySubID {
    value: String,
}
impl Field for Scope {
    fn tag(&self) -> u16 {
        return 546;
    }
}
#[derive(Debug)]
pub enum Scope {
    _Local,
    _National,
    _Global,
}
impl Field for MDImplicitDelete {
    fn tag(&self) -> u16 {
        return 547;
    }
}
#[derive(Debug)]
pub enum MDImplicitDelete {
    _Yes,
    _No,
}
impl Field for CrossID {
    fn tag(&self) -> u16 {
        return 548;
    }
}
pub struct CrossID {
    value: String,
}
impl Field for CrossType {
    fn tag(&self) -> u16 {
        return 549;
    }
}
#[derive(Debug)]
pub enum CrossType { _Crosstradewhichisexecutedcompletelyornotbothsidesaretreatedinthesamemannerthisisequivalenttoanallornone,_Crosstradewhichisexecutedpartiallyandtherestiscancelledonesideisfullyexecutedtheothersideispartiallyexecutedwiththeremainderbeingcancelledthisisequivalenttoanimmediateorcancelontheotherside,_Crosstradewhichispartiallyexecutedwiththeunfilledportionsremainingactiveonesideofthecrossisfullyexecuted,_Crosstradeisexecutedwithexistingorderswiththesameprice }
impl Field for CrossPrioritization {
    fn tag(&self) -> u16 {
        return 550;
    }
}
#[derive(Debug)]
pub enum CrossPrioritization {
    _Sellsideprioritized,
    _None,
    _Buysideprioritized,
}
impl Field for OrigCrossID {
    fn tag(&self) -> u16 {
        return 551;
    }
}
pub struct OrigCrossID {
    value: String,
}
impl Field for NoSides {
    fn tag(&self) -> u16 {
        return 552;
    }
}
#[derive(Debug)]
pub enum NoSides {
    _Oneside,
    _Bothsides,
}
impl Field for Username {
    fn tag(&self) -> u16 {
        return 553;
    }
}
pub struct Username {
    value: String,
}
impl Field for Password {
    fn tag(&self) -> u16 {
        return 554;
    }
}
pub struct Password {
    value: String,
}
impl Field for NoLegs {
    fn tag(&self) -> u16 {
        return 555;
    }
}
pub struct NoLegs {
    value: u16,
}
impl Field for LegCurrency {
    fn tag(&self) -> u16 {
        return 556;
    }
}
pub struct LegCurrency {
    value: f32,
}
impl Field for TotalNumSecurityTypes {
    fn tag(&self) -> u16 {
        return 557;
    }
}
pub struct TotalNumSecurityTypes {
    value: u16,
}
impl Field for NoSecurityTypes {
    fn tag(&self) -> u16 {
        return 558;
    }
}
pub struct NoSecurityTypes {
    value: u16,
}
impl Field for SecurityListRequestType {
    fn tag(&self) -> u16 {
        return 559;
    }
}
#[derive(Debug)]
pub enum SecurityListRequestType {
    _Securitytypeandorcficode,
    _Product,
    _Tradingsessionid,
    _Allsecurities,
    _Symbol,
}
impl Field for SecurityRequestResult {
    fn tag(&self) -> u16 {
        return 560;
    }
}
#[derive(Debug)]
pub enum SecurityRequestResult {
    _Instrumentdatatemporarilyunavailable,
    _Validrequest,
    _Invalidorunsupportedrequest,
    _Requestforinstrumentdatanotsupported,
    _Notauthorizedtoretrieveinstrumentdata,
    _Noinstrumentsfoundthatmatchselectioncriteria,
}
impl Field for RoundLot {
    fn tag(&self) -> u16 {
        return 561;
    }
}
pub struct RoundLot {
    value: f32,
}
impl Field for MinTradeVol {
    fn tag(&self) -> u16 {
        return 562;
    }
}
pub struct MinTradeVol {
    value: f32,
}
impl Field for MultiLegRptTypeReq {
    fn tag(&self) -> u16 {
        return 563;
    }
}
pub struct MultiLegRptTypeReq {
    value: u16,
}
impl Field for LegPositionEffect {
    fn tag(&self) -> u16 {
        return 564;
    }
}
pub struct LegPositionEffect {
    value: char,
}
impl Field for LegCoveredOrUncovered {
    fn tag(&self) -> u16 {
        return 565;
    }
}
pub struct LegCoveredOrUncovered {
    value: u16,
}
impl Field for LegPrice {
    fn tag(&self) -> u16 {
        return 566;
    }
}
pub struct LegPrice {
    value: f32,
}
impl Field for TradSesStatusRejReason {
    fn tag(&self) -> u16 {
        return 567;
    }
}
#[derive(Debug)]
pub enum TradSesStatusRejReason {
    _Unknownorinvalidtradingsessionid,
}
impl Field for TradeRequestID {
    fn tag(&self) -> u16 {
        return 568;
    }
}
pub struct TradeRequestID {
    value: String,
}
impl Field for TradeRequestType {
    fn tag(&self) -> u16 {
        return 569;
    }
}
#[derive(Debug)]
pub enum TradeRequestType {
    _Advisoriesthatmatchcriteria,
    _Unreportedtradesthatmatchcriteria,
    _Unmatchedtradesthatmatchcriteria,
    _Matchedtradesmatchingcriteriaprovidedonrequest,
    _Alltrades,
}
impl Field for PreviouslyReported {
    fn tag(&self) -> u16 {
        return 570;
    }
}
#[derive(Debug)]
pub enum PreviouslyReported {
    _No,
    _Yes,
}
impl Field for TradeReportID {
    fn tag(&self) -> u16 {
        return 571;
    }
}
pub struct TradeReportID {
    value: String,
}
impl Field for TradeReportRefID {
    fn tag(&self) -> u16 {
        return 572;
    }
}
pub struct TradeReportRefID {
    value: String,
}
impl Field for MatchStatus {
    fn tag(&self) -> u16 {
        return 573;
    }
}
#[derive(Debug)]
pub enum MatchStatus {
    _Comparedmatchedoraffirmed,
    _Uncomparedunmatchedorunaffirmed,
    _Advisoryoralert,
}
impl Field for MatchType {
    fn tag(&self) -> u16 {
        return 574;
    }
}
#[derive(Debug)]
pub enum MatchType { _Summarizedmatchusinga1Toa5Exactmatchcriteriaexceptquantityissummarizeds5,_Actm1Match,_Actm6Match,_Actdefaultafterm2,_Actacceptedtrade,_Summarizedmatchusinga1Toa5Exactmatchcriteriaexceptquantityissummarizeds2,_Summarizedmatchusinga1Toa5Exactmatchcriteriaexceptquantityissummarizeds3,_Summarizedmatchusinga1Toa5Exactmatchcriteriaexceptquantityissummarizeds4,_Actm2Match,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusfourbadges,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplustwobadgesandexecutiontime,_Exactmatchontradedatestocksymbolquantitypricetradetypeand,_Comparedrecordsresultingfromstampedadvisoriesorspecialist,_Nonact,_Actdefaulttrade,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusfourbadgesandexecutiontime,_Summarizedmatchusinga1Toa5Exactmatchcriteriaexceptquantityissummarizeds1,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusexecutiontime }
impl Field for OddLot {
    fn tag(&self) -> u16 {
        return 575;
    }
}
pub struct OddLot {
    value: bool,
}
impl Field for NoClearingInstructions {
    fn tag(&self) -> u16 {
        return 576;
    }
}
pub struct NoClearingInstructions {
    value: u16,
}
impl Field for ClearingInstruction {
    fn tag(&self) -> u16 {
        return 577;
    }
}
#[derive(Debug)]
pub enum ClearingInstruction {
    _Manualmode,
    _Multilateralnetting,
    _Automaticpostingmode,
    _Bilateralnettingonly,
    _Clearagainstcentralcounterparty,
    _Automaticgiveupmode,
    _Specialtrade,
    _Exclearing,
    _Processnormally,
    _Excludefromcentralcounterparty,
    _Excludefromallnetting,
}
impl Field for TradeInputSource {
    fn tag(&self) -> u16 {
        return 578;
    }
}
pub struct TradeInputSource {
    value: String,
}
impl Field for TradeInputDevice {
    fn tag(&self) -> u16 {
        return 579;
    }
}
pub struct TradeInputDevice {
    value: String,
}
impl Field for NoDates {
    fn tag(&self) -> u16 {
        return 580;
    }
}
pub struct NoDates {
    value: u16,
}
impl Field for AccountType {
    fn tag(&self) -> u16 {
        return 581;
    }
}
#[derive(Debug)]
pub enum AccountType {
    _Housetrader,
    _Accountishousetraderandiscrossmargined,
    _Accountiscarriedonnoncustomersideofbooksandiscrossmargined,
    _Floortrader,
    _Accountiscarriedonnoncustomersideofbooks,
    _Accountiscarriedoncustomersideofbooks,
    _Jointbackofficeaccount,
}
impl Field for CustOrderCapacity {
    fn tag(&self) -> u16 {
        return 582;
    }
}
pub struct CustOrderCapacity {
    value: u16,
}
impl Field for ClOrdLinkID {
    fn tag(&self) -> u16 {
        return 583;
    }
}
pub struct ClOrdLinkID {
    value: String,
}
impl Field for MassStatusReqID {
    fn tag(&self) -> u16 {
        return 584;
    }
}
pub struct MassStatusReqID {
    value: String,
}
impl Field for MassStatusReqType {
    fn tag(&self) -> u16 {
        return 585;
    }
}
#[derive(Debug)]
pub enum MassStatusReqType {
    _Statusforordersforasecurity,
    _Statusforordersforanunderlyingsecurity,
    _Statusforordersforaproduct,
    _Statusforordersforacficode,
    _Statusforordersforasecuritytype,
    _Statusforordersforatradingsession,
    _Statusforordersforapartyid,
    _Statusforallorders,
}
impl Field for OrigOrdModTime {
    fn tag(&self) -> u16 {
        return 586;
    }
}
pub struct OrigOrdModTime {
    value: u64,
}
impl Field for LegSettlmntTyp {
    fn tag(&self) -> u16 {
        return 587;
    }
}
pub struct LegSettlmntTyp {
    value: char,
}
impl Field for LegFutSettDate {
    fn tag(&self) -> u16 {
        return 588;
    }
}
pub struct LegFutSettDate {
    value: u64,
}
impl Field for DayBookingInst {
    fn tag(&self) -> u16 {
        return 589;
    }
}
#[derive(Debug)]
pub enum DayBookingInst {
    _Cantriggerbookingwithoutreferencetotheorderinitiator,
    _Speakwithorderinitiatorbeforebooking,
}
impl Field for BookingUnit {
    fn tag(&self) -> u16 {
        return 590;
    }
}
#[derive(Debug)]
pub enum BookingUnit {
    _Aggregatepartialexecutionsonthisorderandbookonetradeperorder,
    _Aggregateexecutionsforthissymbolsideandsettlementdate,
    _Eachpartialexecutionisabookableunit,
}
impl Field for PreallocMethod {
    fn tag(&self) -> u16 {
        return 591;
    }
}
#[derive(Debug)]
pub enum PreallocMethod {
    _Prorata,
    _Donotproratadiscussfirst,
}
impl Field for UnderlyingCountryOfIssue {
    fn tag(&self) -> u16 {
        return 592;
    }
}
pub struct UnderlyingCountryOfIssue {
    value: String,
}
impl Field for UnderlyingStateOrProvinceOfIssue {
    fn tag(&self) -> u16 {
        return 593;
    }
}
pub struct UnderlyingStateOrProvinceOfIssue {
    value: String,
}
impl Field for UnderlyingLocaleOfIssue {
    fn tag(&self) -> u16 {
        return 594;
    }
}
pub struct UnderlyingLocaleOfIssue {
    value: String,
}
impl Field for UnderlyingInstrRegistry {
    fn tag(&self) -> u16 {
        return 595;
    }
}
pub struct UnderlyingInstrRegistry {
    value: String,
}
impl Field for LegCountryOfIssue {
    fn tag(&self) -> u16 {
        return 596;
    }
}
pub struct LegCountryOfIssue {
    value: String,
}
impl Field for LegStateOrProvinceOfIssue {
    fn tag(&self) -> u16 {
        return 597;
    }
}
pub struct LegStateOrProvinceOfIssue {
    value: String,
}
impl Field for LegLocaleOfIssue {
    fn tag(&self) -> u16 {
        return 598;
    }
}
pub struct LegLocaleOfIssue {
    value: String,
}
impl Field for LegInstrRegistry {
    fn tag(&self) -> u16 {
        return 599;
    }
}
pub struct LegInstrRegistry {
    value: String,
}
impl Field for LegSymbol {
    fn tag(&self) -> u16 {
        return 600;
    }
}
pub struct LegSymbol {
    value: String,
}
impl Field for LegSymbolSfx {
    fn tag(&self) -> u16 {
        return 601;
    }
}
pub struct LegSymbolSfx {
    value: String,
}
impl Field for LegSecurityID {
    fn tag(&self) -> u16 {
        return 602;
    }
}
pub struct LegSecurityID {
    value: String,
}
impl Field for LegSecurityIDSource {
    fn tag(&self) -> u16 {
        return 603;
    }
}
pub struct LegSecurityIDSource {
    value: String,
}
impl Field for NoLegSecurityAltID {
    fn tag(&self) -> u16 {
        return 604;
    }
}
pub struct NoLegSecurityAltID {
    value: u16,
}
impl Field for LegSecurityAltID {
    fn tag(&self) -> u16 {
        return 605;
    }
}
pub struct LegSecurityAltID {
    value: String,
}
impl Field for LegSecurityAltIDSource {
    fn tag(&self) -> u16 {
        return 606;
    }
}
pub struct LegSecurityAltIDSource {
    value: String,
}
impl Field for LegProduct {
    fn tag(&self) -> u16 {
        return 607;
    }
}
pub struct LegProduct {
    value: u16,
}
impl Field for LegCFICode {
    fn tag(&self) -> u16 {
        return 608;
    }
}
pub struct LegCFICode {
    value: String,
}
impl Field for LegSecurityType {
    fn tag(&self) -> u16 {
        return 609;
    }
}
pub struct LegSecurityType {
    value: String,
}
impl Field for LegMaturityMonthYear {
    fn tag(&self) -> u16 {
        return 610;
    }
}
pub struct LegMaturityMonthYear {
    value: u8,
}
impl Field for LegMaturityDate {
    fn tag(&self) -> u16 {
        return 611;
    }
}
pub struct LegMaturityDate {
    value: u64,
}
impl Field for LegStrikePrice {
    fn tag(&self) -> u16 {
        return 612;
    }
}
pub struct LegStrikePrice {
    value: f32,
}
impl Field for LegOptAttribute {
    fn tag(&self) -> u16 {
        return 613;
    }
}
pub struct LegOptAttribute {
    value: char,
}
impl Field for LegContractMultiplier {
    fn tag(&self) -> u16 {
        return 614;
    }
}
pub struct LegContractMultiplier {
    value: f32,
}
impl Field for LegCouponRate {
    fn tag(&self) -> u16 {
        return 615;
    }
}
pub struct LegCouponRate {
    value: f32,
}
impl Field for LegSecurityExchange {
    fn tag(&self) -> u16 {
        return 616;
    }
}
pub struct LegSecurityExchange {
    value: String,
}
impl Field for LegIssuer {
    fn tag(&self) -> u16 {
        return 617;
    }
}
pub struct LegIssuer {
    value: String,
}
impl Field for EncodedLegIssuerLen {
    fn tag(&self) -> u16 {
        return 618;
    }
}
pub struct EncodedLegIssuerLen {
    value: usize,
}
impl Field for EncodedLegIssuer {
    fn tag(&self) -> u16 {
        return 619;
    }
}
pub struct EncodedLegIssuer {
    value: [u8; 1024],
}
impl Field for LegSecurityDesc {
    fn tag(&self) -> u16 {
        return 620;
    }
}
pub struct LegSecurityDesc {
    value: String,
}
impl Field for EncodedLegSecurityDescLen {
    fn tag(&self) -> u16 {
        return 621;
    }
}
pub struct EncodedLegSecurityDescLen {
    value: usize,
}
impl Field for EncodedLegSecurityDesc {
    fn tag(&self) -> u16 {
        return 622;
    }
}
pub struct EncodedLegSecurityDesc {
    value: [u8; 1024],
}
impl Field for LegRatioQty {
    fn tag(&self) -> u16 {
        return 623;
    }
}
pub struct LegRatioQty {
    value: f32,
}
impl Field for LegSide {
    fn tag(&self) -> u16 {
        return 624;
    }
}
pub struct LegSide {
    value: char,
}
impl Field for TradingSessionSubID {
    fn tag(&self) -> u16 {
        return 625;
    }
}
pub struct TradingSessionSubID {
    value: String,
}
impl Field for AllocType {
    fn tag(&self) -> u16 {
        return 626;
    }
}
#[derive(Debug)]
pub enum AllocType {
    _Buysidereadytobook6,
    _Buysidepreliminary,
    _Sellsidecalculatedusingpreliminary,
    _Buysidereadytobook5,
    _Buysidecalculated,
    _Sellsidecalculatedwithoutpreliminary,
}
impl Field for NoHops {
    fn tag(&self) -> u16 {
        return 627;
    }
}
pub struct NoHops {
    value: u16,
}
impl Field for HopCompID {
    fn tag(&self) -> u16 {
        return 628;
    }
}
pub struct HopCompID {
    value: String,
}
impl Field for HopSendingTime {
    fn tag(&self) -> u16 {
        return 629;
    }
}
pub struct HopSendingTime {
    value: u64,
}
impl Field for HopRefID {
    fn tag(&self) -> u16 {
        return 630;
    }
}
pub struct HopRefID {
    value: u64,
}
impl Field for MidPx {
    fn tag(&self) -> u16 {
        return 631;
    }
}
pub struct MidPx {
    value: f32,
}
impl Field for BidYield {
    fn tag(&self) -> u16 {
        return 632;
    }
}
pub struct BidYield {
    value: f32,
}
impl Field for MidYield {
    fn tag(&self) -> u16 {
        return 633;
    }
}
pub struct MidYield {
    value: f32,
}
impl Field for OfferYield {
    fn tag(&self) -> u16 {
        return 634;
    }
}
pub struct OfferYield {
    value: f32,
}
impl Field for ClearingFeeIndicator {
    fn tag(&self) -> u16 {
        return 635;
    }
}
#[derive(Debug)]
pub enum ClearingFeeIndicator {
    _106Hand106Jfirms,
    _5Thyeardelegatetradingforhisownaccount,
    _4Thyeardelegatetradingforhisownaccount,
    _3Rdyeardelegatetradingforhisownaccount,
    _2Ndyeardelegatetradingforhisownaccount,
    _1Styeardelegatetradingforhisownaccount,
    _Allotherownershiptypes,
    _Gimidemandcommembershipinterestholders,
    _6Thyearandbeyonddelegatetradingforhisownaccount,
    _Fullandassociatemembertradingforownaccountandasfloor,
    _Equitymemberandclearingmember,
    _Nonmemberandcustomer,
    _Cboemember,
    _Lesseeand106Femployees,
}
impl Field for WorkingIndicator {
    fn tag(&self) -> u16 {
        return 636;
    }
}
#[derive(Debug)]
pub enum WorkingIndicator {
    _No,
    _Yes,
}
impl Field for LegLastPx {
    fn tag(&self) -> u16 {
        return 637;
    }
}
pub struct LegLastPx {
    value: f32,
}
impl Field for PriorityIndicator {
    fn tag(&self) -> u16 {
        return 638;
    }
}
#[derive(Debug)]
pub enum PriorityIndicator {
    _Priorityunchanged,
    _Lostpriorityasresultoforderchange,
}
impl Field for PriceImprovement {
    fn tag(&self) -> u16 {
        return 639;
    }
}
pub struct PriceImprovement {
    value: i8,
}
impl Field for Price2 {
    fn tag(&self) -> u16 {
        return 640;
    }
}
pub struct Price2 {
    value: f32,
}
impl Field for LastForwardPoints2 {
    fn tag(&self) -> u16 {
        return 641;
    }
}
pub struct LastForwardPoints2 {
    value: i8,
}
impl Field for BidForwardPoints2 {
    fn tag(&self) -> u16 {
        return 642;
    }
}
pub struct BidForwardPoints2 {
    value: i8,
}
impl Field for OfferForwardPoints2 {
    fn tag(&self) -> u16 {
        return 643;
    }
}
pub struct OfferForwardPoints2 {
    value: i8,
}
impl Field for RFQReqID {
    fn tag(&self) -> u16 {
        return 644;
    }
}
pub struct RFQReqID {
    value: String,
}
impl Field for MktBidPx {
    fn tag(&self) -> u16 {
        return 645;
    }
}
pub struct MktBidPx {
    value: f32,
}
impl Field for MktOfferPx {
    fn tag(&self) -> u16 {
        return 646;
    }
}
pub struct MktOfferPx {
    value: f32,
}
impl Field for MinBidSize {
    fn tag(&self) -> u16 {
        return 647;
    }
}
pub struct MinBidSize {
    value: f32,
}
impl Field for MinOfferSize {
    fn tag(&self) -> u16 {
        return 648;
    }
}
pub struct MinOfferSize {
    value: f32,
}
impl Field for QuoteStatusReqID {
    fn tag(&self) -> u16 {
        return 649;
    }
}
pub struct QuoteStatusReqID {
    value: String,
}
impl Field for LegalConfirm {
    fn tag(&self) -> u16 {
        return 650;
    }
}
#[derive(Debug)]
pub enum LegalConfirm {
    _Yes,
    _No,
}
impl Field for UnderlyingLastPx {
    fn tag(&self) -> u16 {
        return 651;
    }
}
pub struct UnderlyingLastPx {
    value: f32,
}
impl Field for UnderlyingLastQty {
    fn tag(&self) -> u16 {
        return 652;
    }
}
pub struct UnderlyingLastQty {
    value: f32,
}
impl Field for LegRefID {
    fn tag(&self) -> u16 {
        return 654;
    }
}
pub struct LegRefID {
    value: String,
}
impl Field for ContraLegRefID {
    fn tag(&self) -> u16 {
        return 655;
    }
}
pub struct ContraLegRefID {
    value: String,
}
impl Field for SettlCurrBidFxRate {
    fn tag(&self) -> u16 {
        return 656;
    }
}
pub struct SettlCurrBidFxRate {
    value: f32,
}
impl Field for SettlCurrOfferFxRate {
    fn tag(&self) -> u16 {
        return 657;
    }
}
pub struct SettlCurrOfferFxRate {
    value: f32,
}
impl Field for QuoteRequestRejectReason {
    fn tag(&self) -> u16 {
        return 658;
    }
}
#[derive(Debug)]
pub enum QuoteRequestRejectReason {
    _Unknownsymbol,
    _Exchange,
    _Quoterequestexceedslimit,
    _Toolatetoenter,
    _Invalidprice,
    _Notauthorizedtorequestquote,
}
impl Field for SideComplianceID {
    fn tag(&self) -> u16 {
        return 659;
    }
}
pub struct SideComplianceID {
    value: String,
}
