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
    _Perunit,
    _Percentage,
    _Absolute,
    _4,
    _5,
    _Pointsperbondorcontractsupplycontractmultiplier,
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
    _Notheld,
    _Work,
    _Goalong,
    _Overtheday,
    _Held,
    _Participatedontinitiate,
    _Strictscale,
    _Trytoscale,
    _Stayonbidside,
    _Stayonofferside,
    _Nocross,
    _Oktocross,
    _Callfirst,
    _Percentofvolume,
    _Donotincrease,
    _Donotreduce,
    _Allornone,
    _Reinstateonsystemfailure,
    _Institutionsonly,
    _Reinstateontradinghalt,
    _Cancelontradinghalt,
    _Lastpeg,
    _Midpricepeg,
    _Nonnegotiable,
    _Openingpeg,
    _Marketpeg,
    _Cancelonsystemfailure,
    _Primarypeg,
    _Suspend,
    _Customerdisplayinstruction,
    _Netting,
    _Pegtovwap,
    _Tradealong,
    _Trytostop,
    _Cancelifnotbest,
    _Trailingstoppeg,
    _Strictlimit,
    _Ignorepricevaliditychecks,
    _Pegtolimitprice,
    _Worktotargetstrategy,
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
    _Cusip,
    _Sedol,
    _Quik,
    _Isinnumber,
    _Riccode,
    _Isocurrencycode,
    _Isocountrycode,
    _Exchangesymbol,
    _Consolidatedtapeassociation,
    _Bloombergsymbol,
    _Wertpapier,
    _Dutch,
    _Valoren,
    _Sicovam,
    _Belgian,
    _Common,
    _Clearinghouse,
    _Isdafpmlproductspecification,
    _Optionspricereportingauthority,
}
impl Field for IOIID {
    fn tag(&self) -> u16 {
        return 23;
    }
}
pub struct IOIID {
    value: String,
}
impl Field for IOIQltyInd {
    fn tag(&self) -> u16 {
        return 25;
    }
}
#[derive(Debug)]
pub enum IOIQltyInd {
    _Low,
    _Medium,
    _High,
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
    _Small,
    _Medium,
    _Large,
}
impl Field for IOITransType {
    fn tag(&self) -> u16 {
        return 28;
    }
}
#[derive(Debug)]
pub enum IOITransType {
    _New,
    _Cancel,
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
impl Field for NoLinesOfText {
    fn tag(&self) -> u16 {
        return 33;
    }
}
pub struct NoLinesOfText {
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
    _Logon,
    _News,
    _Email,
    _Ordersingle,
    _Orderlist,
    _Ordercancelrequest,
    _Ordercancelreplacerequest,
    _Orderstatusrequest,
    _Allocationinstruction,
    _Listcancelrequest,
    _Listexecute,
    _Liststatusrequest,
    _Liststatus,
    _Allocationinstructionack,
    _Dontknowtrade,
    _Quoterequest,
    _Quote,
    _Settlementinstructions,
    _Marketdatarequest,
    _Marketdatasnapshotfullrefresh,
    _Marketdataincrementalrefresh,
    _Marketdatarequestreject,
    _Quotecancel,
    _Quotestatusrequest,
    _Massquoteacknowledgement,
    _Securitydefinitionrequest,
    _Securitydefinition,
    _Securitystatusrequest,
    _Securitystatus,
    _Tradingsessionstatusrequest,
    _Tradingsessionstatus,
    _Massquote,
    _Businessmessagereject,
    _Bidrequest,
    _Bidresponse,
    _Liststrikeprice,
    _Xmlmessage,
    _Registrationinstructions,
    _Registrationinstructionsresponse,
    _Ordermasscancelrequest,
    _Ordermasscancelreport,
    _Neworders,
    _Crossordercancelreplacerequest,
    _Crossordercancelrequest,
    _Securitytyperequest,
    _Securitytypes,
    _Securitylistrequest,
    _Securitylist,
    _Derivativesecuritylistrequest,
    _Derivativesecuritylist,
    _Neworderab,
    _Multilegordercancelreplace,
    _Tradecapturereportrequest,
    _Tradecapturereport,
    _Ordermassstatusrequest,
    _Quoterequestreject,
    _Rfqrequest,
    _Quotestatusreport,
    _Quoteresponse,
    _Confirmation,
    _Positionmaintenancerequest,
    _Positionmaintenancereport,
    _Requestforpositions,
    _Requestforpositionsack,
    _Positionreport,
    _Tradecapturereportrequestack,
    _Tradecapturereportack,
    _Allocationreport,
    _Allocationreportack,
    _Confirmationack,
    _Settlementinstructionrequest,
    _Assignmentreport,
    _Collateralrequest,
    _Collateralassignment,
    _Collateralresponse,
    _Collateralreport,
    _Collateralinquiry,
    _Networkbc,
    _Networkbd,
    _Userrequest,
    _Userresponse,
    _Collateralinquiryack,
    _Confirmationrequest,
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
    _Filled,
    _Doneforday,
    _Canceled,
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
    _Withorwithout,
    _Limitorbetter,
    _Limitwithorwithout,
    _Onbasis,
    _Previouslyquoted,
    _Previouslyindicated,
    _Forex,
    _Funari,
    _Marketiftouched,
    _Marketwithleftoveraslimit,
    _Previousfundvaluationpoint,
    _Nextfundvaluationpoint,
    _Pegged,
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
    _Yes,
    _No,
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
    _Buy,
    _Sell,
    _Buyminus,
    _Sellplus,
    _Sellshort,
    _Sellshortexempt,
    _Undisclosed,
    _Cross,
    _Crossshort,
    _Crossshortexempt,
    _Asdefined,
    _Opposite,
    _Subscribe,
    _Redeem,
    _Lend,
    _Borrow,
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
    _Day,
    _Goodtillcancel,
    _Attheopening,
    _Immediateorcancel,
    _Fillorkill,
    _Goodtillcrossing,
    _Goodtilldate,
    _Attheclose,
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
impl Field for SettlType {
    fn tag(&self) -> u16 {
        return 63;
    }
}
#[derive(Debug)]
pub enum SettlType {
    _Regular,
    _Cash,
    _Nextday,
    _Tplus2,
    _Tplus3,
    _Tplus4,
    _Future,
    _Whenandifissued,
    _Sellersoption,
    _Tplus5,
}
impl Field for SettlDate {
    fn tag(&self) -> u16 {
        return 64;
    }
}
pub struct SettlDate {
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
impl Field for AvgPxPrecision {
    fn tag(&self) -> u16 {
        return 74;
    }
}
pub struct AvgPxPrecision {
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
    _Open,
    _Close,
    _Rolled,
    _Fifo,
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
    value: f32,
}
impl Field for NoDlvyInst {
    fn tag(&self) -> u16 {
        return 85;
    }
}
pub struct NoDlvyInst {
    value: u16,
}
impl Field for AllocStatus {
    fn tag(&self) -> u16 {
        return 87;
    }
}
#[derive(Debug)]
pub enum AllocStatus {
    _Accepted,
    _Blocklevelreject,
    _Accountlevelreject,
    _Received,
    _Incomplete,
    _Rejectedbyintermediary,
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
    _Incorrectallocatedquantity,
    _Calculationdifference,
    _Unknownorstaleexecid,
    _Mismatcheddatavalue,
    _Unknownclordid,
    _Warehouserequestrejected,
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
    _Yes,
    _No,
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
    _Toolatetocancel,
    _Unknownorder,
    _Broker,
    _Orderalreadyinpendingcancelorpendingreplacestatus,
    _Unabletoprocessordermasscancelrequest,
    _Origordmodtime,
    _Duplicateclordid,
    _Other,
}
impl Field for OrdRejReason {
    fn tag(&self) -> u16 {
        return 103;
    }
}
#[derive(Debug)]
pub enum OrdRejReason {
    _Broker,
    _Unknownsymbol,
    _Exchangeclosed,
    _Orderexceedslimit,
    _Toolatetoenter,
    _Unknownorder,
    _Duplicateorder,
    _Duplicateofaverballycommunicatedorder,
    _Staleorder,
    _Tradealongrequired,
    _Invalidinvestorid,
    _Unsupportedordercharacteristic12Surveillenceoption,
    _Incorrectquantity,
    _Incorrectallocatedquantity,
    _Unknownaccount,
    _Other,
}
impl Field for IOIQualifier {
    fn tag(&self) -> u16 {
        return 104;
    }
}
#[derive(Debug)]
pub enum IOIQualifier {
    _Allornone,
    _Marketonclose,
    _Attheclose,
    _Vwap,
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
    _Unknownsymbol,
    _Wrongside,
    _Quantityexceedsorder,
    _Nomatchingorder,
    _Priceexceedslimit,
    _Calculationdifference,
    _Other,
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
    _Regulatory,
    _Tax,
    _Localcommission,
    _Exchangefees,
    _Stamp,
    _Levy,
    _Other,
    _Markup,
    _Consumptiontax,
    _Pertransaction,
    _Conversion,
    _Agent,
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
    _New,
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
    _Trade,
    _Tradecorrect,
    _Tradecancel,
    _Orderstatus,
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
    _Multiply,
    _Divide,
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
    _Standinginstructionsprovided,
    _Specificorderforasingleaccount,
    _Requestreject,
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
    _Cancel,
    _Replace,
    _Restate,
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
    _Brokersinstructions,
    _Institutionsinstructions,
    _Investor,
}
impl Field for SecurityType {
    fn tag(&self) -> u16 {
        return 167;
    }
}
#[derive(Debug)]
pub enum SecurityType {
    _Future,
    _Option,
    _Eurosupranationalcoupons,
    _Federalagencycoupon,
    _Federalagencydiscountnote,
    _Privateexportfunding,
    _Usdsupranationalcoupons,
    _Corporatebond,
    _Corporateprivateplacement,
    _Convertiblebond,
    _Dualcurrency,
    _Eurocorporatebond,
    _Indexedlinked,
    _Structurednotes,
    _Yankeecorporatebond,
    _Foreignexchangecontract,
    _Commonstock,
    _Preferredstock,
    _Bradybond,
    _Eurosovereigns,
    _Ustreasurybond,
    _Intereststripfromanybondornote,
    _Treasuryinflationprotectedsecurities,
    _Principalstripofacallablebondornote,
    _Principalstripfromanoncallablebondornote,
    _Ustreasurynoteust,
    _Ustreasurybillustb,
    _Ustreasurynotetnote,
    _Ustreasurybilltbill,
    _Repurchase,
    _Forward,
    _Buysellback,
    _Securitiesloan,
    _Securitiespledge,
    _Termloan,
    _Revolverloan,
    _Revolvertermloan,
    _Bridgeloan,
    _Letterofcredit,
    _Swinglinefacility,
    _Debtorinpossession,
    _Defaulted,
    _Withdrawn,
    _Replaced,
    _Matured,
    _Amendedrestated,
    _Retired,
    _Bankersacceptance,
    _Banknotes,
    _Billofexchanges,
    _Certificateofdeposit,
    _Callloans,
    _Commercialpaper,
    _Depositnotes,
    _Eurocertificateofdeposit,
    _Eurocommercialpaper,
    _Liquiditynote,
    _Mediumtermnotes,
    _Overnight,
    _Promissorynote,
    _Plazosfijos,
    _Shorttermloannote,
    _Timedeposit,
    _Extendedcommnote,
    _Yankeecertificateofdeposit,
    _Assetbackedsecurities,
    _Corpmortgagebackedsecurities,
    _Collateralizedmortgageobligation,
    _Ioettemortgage,
    _Mortgagebackedsecurities,
    _Mortgageinterestonly,
    _Mortgageprincipalonly,
    _Mortgageprivateplacement,
    _Miscellaneouspassthrough,
    _Pfandbriefe,
    _Tobeannounced,
    _Otheranticipationnotesbanganetc,
    _Certificateofobligation,
    _Certificateofparticipation,
    _Generalobligationbonds,
    _Mandatorytender,
    _Revenueanticipationnote,
    _Revenuebonds,
    _Specialassessment,
    _Specialobligation,
    _Specialtax,
    _Taxanticipationnote,
    _Taxallocation,
    _Taxexemptcommercialpaper,
    _Taxrevenueanticipationnote,
    _Variableratedemandnote,
    _Warrant,
    _Mutualfund,
    _Multileginstrument,
    _Nosecuritytype,
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
    _Thomsonalert,
    _Aglobalcustodian,
    _Accountnet,
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
    _Versuspaymentdeliver,
    _Freedeliver,
    _Triparty,
    _Holdincustody,
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
impl Field for SettlDate2 {
    fn tag(&self) -> u16 {
        return 193;
    }
}
pub struct SettlDate2 {
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
impl Field for PutOrCall {
    fn tag(&self) -> u16 {
        return 201;
    }
}
#[derive(Debug)]
pub enum PutOrCall {
    _Put,
    _Call,
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
    _Covered,
    _Uncovered,
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
    _Yes,
    _No,
}
impl Field for AllocHandlInst {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 210;
    }
}
pub struct MaxShow {
    value: f32,
}
impl Field for PegOffsetValue {
    fn tag(&self) -> u16 {
        return 211;
    }
}
pub struct PegOffsetValue {
    value: f32,
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
pub struct BenchmarkCurveName {
    value: String,
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
    value: u64,
}
impl Field for IssueDate {
    fn tag(&self) -> u16 {
        return 225;
    }
}
pub struct IssueDate {
    value: u64,
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
    value: u64,
}
impl Field for ExDate {
    fn tag(&self) -> u16 {
        return 230;
    }
}
pub struct ExDate {
    value: u64,
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
    _Amt,
    _Autoreinvestmentatrateorbetter,
    _Bankqualified,
    _Bargainconditionssee,
    _Couponrange,
    _Isocurrencycode,
    _Customstartenddate,
    _Geographicsandrange,
    _Valuationdiscount,
    _Insured,
    _Yearoryearmonthofissue,
    _Issuersticker,
    _Issuesizerange,
    _Lookbackdays,
    _Explicitlotidentifier,
    _Lotvariance,
    _Maturityyearandmonth,
    _Maturityrange,
    _Maximumsubstitutions,
    _Minimumquantity,
    _Minimumincrement,
    _Minimumdenomination,
    _Paymentfrequencycalendar,
    _Numberofpieces,
    _Poolsmaximum,
    _Poolspermillion,
    _Poolsperlot,
    _Poolspertrade,
    _Pricerange,
    _Pricingfrequency,
    _Productionyear,
    _Callprotection,
    _Purpose,
    _Benchmarkpricesource,
    _Ratingsourceandrange,
    _Typeofredemptionvaluesarenoncallablecallableprefundedescrowedtomaturityputableconvertible,
    _Restricted,
    _Marketsector,
    _Securitytypeincludedorexcluded,
    _Structure,
    _Substitutionsfrequency,
    _Substitutionsleft,
    _Freeformtext,
    _Tradevariance,
    _Weightedaveragecouponvalueinpercent,
    _Weightedaveragelifecouponvalueinpercent,
    _Weightedaverageloanagevalueinmonths,
    _Weightedaveragematurityvalueinmonths,
    _Wholepool,
    _Yieldrange,
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
pub enum YieldType {
    _Aftertaxyield,
    _Annualyield,
    _Yieldatissue,
    _Yieldtoaveragematurity,
    _Bookyield,
    _Yieldtonextcall,
    _Yieldchangesinceclose,
    _Closingyield,
    _Compoundyield,
    _Currentyield,
    _Truegrossyield,
    _Governmentequivalentyield,
    _Yieldwithinflationassumption,
    _Inversefloaterbondyield,
    _Mostrecentclosingyield,
    _Closingyieldmostrecentmonth,
    _Closingyieldmostrecentquarter,
    _Closingyieldmostrecentyear,
    _Yieldtolongestaveragelife,
    _Marktomarketyield,
    _Yieldtomaturity,
    _Yieldtonextrefund,
    _Openaverageyield,
    _Yieldtonextput,
    _Previouscloseyield,
    _Proceedsyield,
    _Semiannualyield,
    _Yieldtoshortestaveragelife,
    _Simpleyield,
    _Taxequivalentyield,
    _Yieldtotenderdate,
    _Trueyield,
    _Yieldvalueof132,
    _Yieldtoworst,
}
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
    value: u64,
}
impl Field for UnderlyingCouponPaymentDate {
    fn tag(&self) -> u16 {
        return 241;
    }
}
pub struct UnderlyingCouponPaymentDate {
    value: u64,
}
impl Field for UnderlyingIssueDate {
    fn tag(&self) -> u16 {
        return 242;
    }
}
pub struct UnderlyingIssueDate {
    value: u64,
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
    value: u64,
}
impl Field for LegCouponPaymentDate {
    fn tag(&self) -> u16 {
        return 248;
    }
}
pub struct LegCouponPaymentDate {
    value: u64,
}
impl Field for LegIssueDate {
    fn tag(&self) -> u16 {
        return 249;
    }
}
pub struct LegIssueDate {
    value: u64,
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
    value: u64,
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
    _Yes,
    _No,
}
impl Field for BasisFeatureDate {
    fn tag(&self) -> u16 {
        return 259;
    }
}
pub struct BasisFeatureDate {
    value: u64,
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
    _Snapshot,
    _Snapshotplusupdates,
    _Disableprevioussnapshotplusupdaterequest,
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
    _Imbalance,
    _Tradevolume,
    _Openinterest,
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
    value: u16,
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
    fn tag(&self) -> u16 {
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
    _Imbalancemorebuyers,
    _Imbalancemoresellers,
    _Openingprice,
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
    _Unknownsymbol,
    _Duplicatemdreqid,
    _Insufficientbandwidth,
    _Insufficientpermissions,
    _Unsupportedsubscriptionrequesttype,
    _Unsupportedmarketdepth,
    _Unsupportedmdupdatetype,
    _Unsupportedaggregatedbook,
    _Unsupportedmdentrytype,
    _Unsupportedtradingsessionid,
    _Unsupportedscope,
    _Unsupportedopenclosesettleflag,
    _Unsupportedmdimplicitdelete,
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
impl Field for OpenCloseSettlFlag {
    fn tag(&self) -> u16 {
        return 286;
    }
}
#[derive(Debug)]
pub enum OpenCloseSettlFlag {
    _Dailyopen,
    _Sessionopen,
    _Deliverysettlemententry,
    _Expectedentry,
    _Entryfrompreviousbusinessday,
    _Theoreticalpricevalue,
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
    _Exdividend,
    _Exdistribution,
    _Exrights,
    _New,
    _Exinterest,
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
    _Accepted,
    _Canceledforsymbol,
    _Canceledforsecuritytype,
    _Canceledforunderlying,
    _Canceledall,
    _Rejected,
    _Removedfrommarket,
    _Expired,
    _Query,
    _Quotenotfound,
    _Pending,
    _Pass,
    _Lockedmarketwarning,
    _Crossmarketwarning,
    _Canceledduetolockmarket,
    _Canceledduetocrossmarket,
}
impl Field for QuoteCancelType {
    fn tag(&self) -> u16 {
        return 298;
    }
}
#[derive(Debug)]
pub enum QuoteCancelType {
    _Cancelforsymbol,
    _Cancelforsecuritytype,
    _Cancelforunderlyingsymbol,
    _Cancelallquotes,
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
    _Unknownsymbol,
    _Exchange,
    _Quoterequestexceedslimit,
    _Toolatetoenter,
    _Unknownquote,
    _Duplicatequote,
    _Invalidbidaskspread,
    _Invalidprice,
    _Notauthorizedtoquotesecurity,
    _Other,
}
impl Field for QuoteResponseLevel {
    fn tag(&self) -> u16 {
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
    _Manual,
    _Automatic,
}
impl Field for TotNoQuoteEntries {
    fn tag(&self) -> u16 {
        return 304;
    }
}
pub struct TotNoQuoteEntries {
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
impl Field for UnderlyingCurrency {
    fn tag(&self) -> u16 {
        return 318;
    }
}
pub struct UnderlyingCurrency {
    value: f32,
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
    _Acceptsecurityproposalasis,
    _Acceptsecurityproposalwithrevisionsasindicatedinthemessage,
    _Rejectsecurityproposal,
    _Cannotmatchselectioncriteria,
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
    _Openingdelay,
    _Tradinghalt,
    _Resume,
    _Noopennoresume,
    _Priceindication,
    _Tradingrangeindication,
    _Marketimbalancebuy,
    _Marketimbalancesell,
    _Marketoncloseimbalancebuy,
    _Marketoncloseimbalancesell,
    _Nomarketimbalance,
    _Nomarketoncloseimbalance,
    _Itspreopening,
    _Newpriceindication,
    _Tradedisseminationtime,
    _Readytotrade,
    _Notavailablefortrading,
    _Nottradedonthismarket,
    _Unknownorinvalid,
    _Preopen,
    _Openingrotation,
    _Fastmarket,
}
impl Field for HaltReasonChar {
    fn tag(&self) -> u16 {
        return 327;
    }
}
#[derive(Debug)]
pub enum HaltReasonChar {
    _Orderimbalance,
    _Equipmentchangeover,
    _Newspending,
    _Newsdissemination,
    _Orderinflux,
    _Additionalinformation,
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
    _Electronic,
    _Openoutcry,
    _Twoparty,
}
impl Field for TradSesMode {
    fn tag(&self) -> u16 {
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
    fn tag(&self) -> u16 {
        return 340;
    }
}
#[derive(Debug)]
pub enum TradSesStatus {
    _Unknown,
    _Halted,
    _Open,
    _Closed,
    _Preopen,
    _Preclose,
    _Requestrejected,
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
    _Jis,
    _Euc,
    _Forusingsjis,
    _Unicode,
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
    _Invalidtagnumber,
    _Requiredtagmissing,
    _Tagnotdefinedforthismessagetype,
    _Undefinedtag,
    _Tagspecifiedwithoutavalue,
    _Valueisincorrect,
    _Incorrectdataformatforvalue,
    _Decryptionproblem,
    _Signatureproblem,
    _Compidproblem,
    _Sendingtimeaccuracyproblem,
    _Invalidmsgtype,
    _Xmlvalidationerror,
    _Tagappearsmorethanonce,
    _Tagspecifiedoutofrequiredorder,
    _Repeatinggroupfieldsoutoforder,
    _Incorrectnumingroupcountforrepeatinggroup,
    _Nondatavalueincludesfielddelimiter,
    _Other,
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
    _Yes,
    _No,
}
impl Field for ExecRestatementReason {
    fn tag(&self) -> u16 {
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
    _Cancelontradinghalt,
    _Cancelonsystemfailure,
    _Market,
    _Cancelednotbest,
    _Warehouserecap,
    _Other,
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
    _Other,
    _Unkownid,
    _Unknownsecurity,
    _Unsupportedmessagetype,
    _Applicationnotavailable,
    _Conditionallyrequiredfieldmissing,
    _Notauthorized,
    _Delivertofirmnotavailableatthistime,
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
    _Relatedtovwap,
}
impl Field for DiscretionOffsetValue {
    fn tag(&self) -> u16 {
        return 389;
    }
}
pub struct DiscretionOffsetValue {
    value: f32,
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
impl Field for TotNoRelatedSym {
    fn tag(&self) -> u16 {
        return 393;
    }
}
pub struct TotNoRelatedSym {
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
    _Sector,
    _Country,
    _Index,
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
    _5Daymovingaverage,
    _20Daymovingaverage,
    _Normalmarketsize,
    _Other,
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
    _Yes,
    _No,
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
    _Buysideexplicitlyrequestsstatususingstatusrequest,
    _Sellsideperiodicallysendsstatususingliststatusperiodoptionallyspecifiedinprogressperiod,
    _Realtimeexecutionreports,
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
    _Net,
    _Gross,
}
impl Field for NumBidders {
    fn tag(&self) -> u16 {
        return 417;
    }
}
pub struct NumBidders {
    value: u16,
}
impl Field for BidTradeType {
    fn tag(&self) -> u16 {
        return 418;
    }
}
#[derive(Debug)]
pub enum BidTradeType {
    _Risktrade,
    _Vwapguarantee,
    _Agency,
    _Guaranteedclose,
}
impl Field for BasisPxType {
    fn tag(&self) -> u16 {
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
    _Percentage,
    _Perunit,
    _Fixedamount,
    _Discountpercentagepointsbelowpar,
    _Premiumpercentagepointsoverpar,
    _Spread,
    _Tedprice,
    _Tedyield,
    _Yield,
    _Fixedcabinettradeprice,
    _Variablecabinettradeprice,
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
    _Accumulateexecutionsuntilorderisfilledorexpires,
    _Accumulateuntilverballynotifiedotherwise,
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
    _Ack,
    _Response,
    _Timed,
    _Execstarted,
    _Alldone,
    _Alert,
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
    _Inbiddingprocess,
    _Receivedforexecution,
    _Executing,
    _Canceling,
    _Alert,
    _Alldone,
    _Reject,
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
    _Immediate,
    _Waitforexecuteinstruction,
    _Exchangeswitchcivorderselldriven,
    _Exchangeswitchcivorderbuydrivencashtopup,
    _Exchangeswitchcivorderbuydrivencashwithdraw,
}
impl Field for CxlRejResponseTo {
    fn tag(&self) -> u16 {
        return 434;
    }
}
#[derive(Debug)]
pub enum CxlRejResponseTo {
    _Ordercancelrequest,
    _Ordercancelreplacerequest,
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
    _Bic,
    _Generallyacceptedmarketparticipantidentifier,
    _Proprietarycustomcode,
    _Isocountrycode,
    _Settlemententitylocation,
    _Mic,
    _Csdparticipantmembercode,
    _Koreaninvestorid,
    _Taiwanesequalifiedforeigninvestoridqfii,
    _Taiwanesetradingaccount,
    _Malaysiancentraldepository,
    _Chinesebshare,
    _Uknationalinsuranceorpensionnumber,
    _Ussocialsecuritynumber,
    _Usemployeridentificationnumber,
    _Australianbusinessnumber,
    _Australiantaxfilenumber,
    _Directedbrokerthreecharacteracronymasdefinedinisitcetcbestpracticeguidelinesdocument,
}
impl Field for PartyID {
    fn tag(&self) -> u16 {
        return 448;
    }
}
pub struct PartyID {
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
    _Executingfirm,
    _Brokerofcredit,
    _Clientid,
    _Clearingfirm,
    _Investorid,
    _Introducingfirm,
    _Enteringfirm,
    _Locatelendingfirm,
    _Fundmanagerclientid,
    _Settlementlocation,
    _Orderoriginationtrader,
    _Executingtrader,
    _Orderoriginationfirm,
    _Giveupclearingfirm,
    _Correspondantclearingfirm,
    _Executingsystem,
    _Contrafirm,
    _Contraclearingfirm,
    _Sponsoringfirm,
    _Underlyingcontrafirm,
    _Clearingorganization,
    _Exchange,
    _Customeraccount,
    _Correspondentclearingorganization,
    _Correspondentbroker,
    _Buyerseller,
    _Custodian,
    _Intermediary,
    _Agent,
    _Subcustodian,
    _Beneficiary,
    _Interestedparty,
    _Regulatorybody,
    _Liquidityprovider,
    _Enteringtrader,
    _Contratrader,
    _Positionaccount,
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
    _Agency,
    _Commodity,
    _Corporate,
    _Currency,
    _Equity,
    _Government,
    _Index,
    _Loan,
    _Moneymarket,
    _Mortgage,
    _Municipal,
    _Other,
    _Financing,
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
#[derive(Debug)]
pub enum DistribPaymentMethod {
    _Crest,
    _Nscc,
    _Euroclear,
    _Clearstream,
    _Cheque,
    _Telegraphictransfer,
    _Fedwire,
    _Directcredit,
    _Achcredit,
    _Bpay,
    _Highvalueclearingsystem,
    _Reinvestinfund,
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
    _Yes,
    _Noexecutiononly,
    _Nowaiveragreement,
    _Noinstitutional,
}
impl Field for MoneyLaunderingStatus {
    fn tag(&self) -> u16 {
        return 481;
    }
}
#[derive(Debug)]
pub enum MoneyLaunderingStatus {
    _Passed,
    _Notchecked,
    _Exemptbelowthelimit,
    _Exemptclientmoneytypeexemption,
    _Exemptauthorisedcreditorfinancialinstitution,
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
    _Bidprice,
    _Creationprice,
    _Creationpriceplusadjustment,
    _Creationpriceplusadjustmentamount,
    _Offerprice,
    _Offerpriceminusadjustment,
    _Offerpriceminusadjustmentamount,
    _Singleprice,
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
pub struct TradeReportTransType {
    value: u16,
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
impl Field for CardIssNum {
    fn tag(&self) -> u16 {
        return 491;
    }
}
pub struct CardIssNum {
    value: String,
}
impl Field for PaymentMethod {
    fn tag(&self) -> u16 {
        return 492;
    }
}
#[derive(Debug)]
pub enum PaymentMethod {
    _Crest,
    _Nscc,
    _Euroclear,
    _Clearstream,
    _Cheque,
    _Telegraphictransfer,
    _Fedwire,
    _Debitcard,
    _Directdebit,
    _Directcredit,
    _Creditcard,
    _Achdebit,
    _Achcredit,
    _Bpay,
    _Highvalueclearingsystem,
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
    _Nonenotapplicable,
    _Maxiisa,
    _Tessa,
    _Minicashisa,
    _Ministocksandsharesisa,
    _Miniinsuranceisa,
    _Currentyearpayment,
    _Prioryearpayment,
    _Assettransfer,
    _Employee,
    _Employeecurrentyear,
    _Employer,
    _Employercurrentyear,
    _Nonfundprototypeira,
    _Nonfundqualifiedplan,
    _Definedcontributionplan,
    _Individualretirementaccount,
    _Individualretirementaccountrollover,
    _Keogh,
    _Profitsharingplan,
    _401K,
    _Selfdirectedira,
    _403,
    _457,
    _Rothira24,
    _Rothira25,
    _Rothconversionira26,
    _Rothconversionira27,
    _Educationira28,
    _Educationira29,
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
    _Yes,
    _No,
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
impl Field for CashDistribAgentAcctName {
    fn tag(&self) -> u16 {
        return 502;
    }
}
pub struct CashDistribAgentAcctName {
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
    _Accepted,
    _Rejected,
    _Held,
    _Reminderieregistrationinstructionsarestilloutstanding,
}
impl Field for RegistRejReasonCode {
    fn tag(&self) -> u16 {
        return 507;
    }
}
#[derive(Debug)]
pub enum RegistRejReasonCode {
    _Invalidunacceptableaccounttype,
    _Invalidunacceptabletaxexempttype,
    _Invalidunacceptableownershiptype,
    _Invalidunacceptablenoregdetls,
    _Invalidunacceptableregseqno,
    _Invalidunacceptableregdtls,
    _Invalidunacceptablemailingdtls,
    _Invalidunacceptablemailinginst,
    _Invalidunacceptableinvestorid,
    _Invalidunacceptableinvestoridsource,
    _Invalidunacceptabledateofbirth,
    _Invalidunacceptableinvestorcountryofresidence,
    _Invalidunacceptablenodistribinstns,
    _Invalidunacceptabledistribpercentage,
    _Invalidunacceptabledistribpaymentmethod,
    _Invalidunacceptablecashdistribagentacctname,
    _Invalidunacceptablecashdistribagentcode,
    _Invalidunacceptablecashdistribagentacctnum,
    _Other,
}
impl Field for RegistRefID {
    fn tag(&self) -> u16 {
        return 508;
    }
}
pub struct RegistRefID {
    value: String,
}
impl Field for RegistDtls {
    fn tag(&self) -> u16 {
        return 509;
    }
}
pub struct RegistDtls {
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
    _New,
    _Replace,
    _Cancel,
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
#[derive(Debug)]
pub enum OwnershipType {
    _Jointinvestors,
    _Tenantsincommon,
    _Jointtrustees,
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
    _Fundbasedrenewalcommissionamount13,
    _Fundbasedrenewalcommissionamount14,
    _Netsettlementamount,
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
    _Individualinvestor,
    _Publiccompany,
    _Privatecompany,
    _Individualtrustee,
    _Companytrustee,
    _Pensionplan,
    _Custodianundergiftstominorsact,
    _Trusts,
    _Fiduciaries,
    _Networkingsubaccount,
    _Nonprofitorganization,
    _Corporatebody,
    _Nominee,
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
    _Agency,
    _Proprietary,
    _Individual,
    _Principal,
    _Risklessprincipal,
    _Agentforothermember,
}
impl Field for OrderRestrictions {
    fn tag(&self) -> u16 {
        return 529;
    }
}
#[derive(Debug)]
pub enum OrderRestrictions {
    _Programtrade,
    _Indexarbitrage,
    _Nonindexarbitrage,
    _Competingmarketmaker,
    _Actingasmarketmakerorspecialistinthesecurity,
    _Actingasmarketmakerorspecialistintheunderlyingsecurityofaderivativesecurity,
    _Foreignentity,
    _Externalmarketparticipant,
    _Externalinterconnectedmarketlinkage,
    _Risklessarbitrage,
}
impl Field for MassCancelRequestType {
    fn tag(&self) -> u16 {
        return 530;
    }
}
#[derive(Debug)]
pub enum MassCancelRequestType {
    _Cancelordersforasecurity,
    _Cancelordersforanunderlyingsecurity,
    _Cancelordersforaproduct,
    _Cancelordersforacficode,
    _Cancelordersforasecuritytype,
    _Cancelordersforatradingsession,
    _Cancelallorders,
}
impl Field for MassCancelResponse {
    fn tag(&self) -> u16 {
        return 531;
    }
}
#[derive(Debug)]
pub enum MassCancelResponse {
    _Cancelrequestrejected,
    _Cancelordersforasecurity,
    _Cancelordersforanunderlyingsecurity,
    _Cancelordersforaproduct,
    _Cancelordersforacficode,
    _Cancelordersforasecuritytype,
    _Cancelordersforatradingsession,
    _Cancelallorders,
}
impl Field for MassCancelRejectReason {
    fn tag(&self) -> u16 {
        return 532;
    }
}
#[derive(Debug)]
pub enum MassCancelRejectReason {
    _Masscancelnotsupported,
    _Invalidorunknownsecurity,
    _Invalidorunknownunderlying,
    _Invalidorunknownproduct,
    _Invalidorunknowncficode,
    _Invalidorunknownsecuritytype,
    _Invalidorunknowntradingsession,
    _Other,
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
    _Counter,
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
    _Cash,
    _Marginopen,
    _Marginclose,
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
pub enum CrossType { _Crosstradewhichisexecutedcompletelyornotbothsidesaretreatedinthesamemannerthisisequivalenttoanallornone,_Crosstradewhichisexecutedpartiallyandtherestiscancelledonesideisfullyexecutedtheothersideispartiallyexecutedwiththeremainderbeingcancelledthisisequivalenttoanimmediateorcancelontheothersidenotethecrossprioritzation,_Crosstradewhichispartiallyexecutedwiththeunfilledportionsremainingactiveonesideofthecrossisfullyexecuted,_Crosstradeisexecutedwithexistingorderswiththesamepriceinthecaseotherordersexistwiththesamepricethequantityofthecrossisexecutedagainsttheexistingordersandquotestheremainderofthecrossisexecutedagainsttheothersideofthecrossthetwosidespotentiallyhavedifferentquantities }
impl Field for CrossPrioritization {
    fn tag(&self) -> u16 {
        return 550;
    }
}
#[derive(Debug)]
pub enum CrossPrioritization {
    _None,
    _Buysideisprioritized,
    _Sellsideisprioritized,
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
impl Field for TotNoSecurityTypes {
    fn tag(&self) -> u16 {
        return 557;
    }
}
pub struct TotNoSecurityTypes {
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
    _Symbol,
    _Securitytypeandorcficode,
    _Product,
    _Tradingsessionid,
    _Allsecurities,
}
impl Field for SecurityRequestResult {
    fn tag(&self) -> u16 {
        return 560;
    }
}
#[derive(Debug)]
pub enum SecurityRequestResult {
    _Validrequest,
    _Invalidorunsupportedrequest,
    _Noinstrumentsfoundthatmatchselectioncriteria,
    _Notauthorizedtoretrieveinstrumentdata,
    _Instrumentdatatemporarilyunavailable,
    _Requestforinstrumentdatanotsupported,
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
#[derive(Debug)]
pub enum MultiLegRptTypeReq {
    _Reportbymulitlegsecurityonly,
    _Reportbymultilegsecurityandbyinstrumentlegsbelongingtothemultilegsecurity,
    _Reportbyinstrumentlegsbelongingtothemultilegsecurityonly,
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
    _Other,
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
    _Alltrades,
    _Matchedtradesmatchingcriteriaprovidedonrequest,
    _Unmatchedtradesthatmatchcriteria,
    _Unreportedtradesthatmatchcriteria,
    _Advisoriesthatmatchcriteria,
}
impl Field for PreviouslyReported {
    fn tag(&self) -> u16 {
        return 570;
    }
}
#[derive(Debug)]
pub enum PreviouslyReported {
    _Yes,
    _No,
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
pub enum MatchType { _Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusfourbadgesandexecutiontime,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusfourbadges,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplustwobadgesandexecutiontime,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplustwobadges,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusexecutiontime,_Comparedrecordsresultingfromstampedadvisoriesorspecialistacceptspairoffs,_Summarizedmatchusinga1Exactmatchcriteriaexceptquantityissummarized,_Summarizedmatchusinga2Exactmatchcriteriaexceptquantityissummarized,_Summarizedmatchusinga3Exactmatchcriteriaexceptquantityissummarized,_Summarizedmatchusinga4Exactmatchcriteriaexceptquantityissummarized,_Summarizedmatchusinga5Exactmatchcriteriaexceptquantityissummarized,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorminusbadgesandtimesactm1Match,_Summarizedmatchminusbadgesandtimesactm2Match,_Ocslockedinnonact,_Actacceptedtrade,_Actdefaulttrade,_Actdefaultafterm2,_Actm6Match }
impl Field for OddLot {
    fn tag(&self) -> u16 {
        return 575;
    }
}
#[derive(Debug)]
pub enum OddLot {
    _Yes,
    _No,
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
    _Processnormally,
    _Excludefromallnetting,
    _Bilateralnettingonly,
    _Exclearing,
    _Specialtrade,
    _Multilateralnetting,
    _Clearagainstcentralcounterparty,
    _Excludefromcentralcounterparty,
    _Manualmode,
    _Automaticpostingmode,
    _Automaticgiveupmode,
    _Qualifiedservicerepresentative,
    _Customertrade,
    _Selfclearing,
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
    _Accountiscarriedoncustomersideofbooks,
    _Accountiscarriedonnoncustomersideofbooks,
    _Housetrader,
    _Floortrader,
    _Accountiscarriedonnoncustomersideofbooksandiscrossmargined,
    _Accountishousetraderandiscrossmargined,
    _Jointbackofficeaccount,
}
impl Field for CustOrderCapacity {
    fn tag(&self) -> u16 {
        return 582;
    }
}
#[derive(Debug)]
pub enum CustOrderCapacity {
    _Membertradingfortheirownaccount,
    _Clearingfirmtradingforitsproprietaryaccount,
    _Membertradingforanothermember,
    _Allother,
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
    _Statusforallorders,
    _Statusforordersforapartyid,
}
impl Field for OrigOrdModTime {
    fn tag(&self) -> u16 {
        return 586;
    }
}
pub struct OrigOrdModTime {
    value: u64,
}
impl Field for LegSettlType {
    fn tag(&self) -> u16 {
        return 587;
    }
}
pub struct LegSettlType {
    value: char,
}
impl Field for LegSettlDate {
    fn tag(&self) -> u16 {
        return 588;
    }
}
pub struct LegSettlDate {
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
    _Accumulate,
}
impl Field for BookingUnit {
    fn tag(&self) -> u16 {
        return 590;
    }
}
#[derive(Debug)]
pub enum BookingUnit {
    _Eachpartialexecutionisabookableunit,
    _Aggregatepartialexecutionsonthisorderandbookonetradeperorder,
    _Aggregateexecutionsforthissymbolsideandsettlementdate,
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
    _Calculated,
    _Preliminary,
    _Readytobook,
    _Warehouseinstruction,
    _Requesttointermediary,
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
    _Cboemember,
    _Nonmemberandcustomer,
    _Equitymemberandclearingmember,
    _Fullandassociatemembertradingforownaccountandasfloorbrokers,
    _106Hand106Jfirms,
    _Gimidemandcommembershipinterestholders,
    _Lesseeand106Femployees,
    _Allotherownershiptypes,
    _1Styeardelegatetradingforhisownaccount,
    _2Ndyeardelegatetradingforhisownaccount,
    _3Rdyeardelegatetradingforhisownaccount,
    _4Thyeardelegatetradingforhisownaccount,
    _5Thyeardelegatetradingforhisownaccount,
    _6Thyearandbeyonddelegatetradingforhisownaccount,
}
impl Field for WorkingIndicator {
    fn tag(&self) -> u16 {
        return 636;
    }
}
#[derive(Debug)]
pub enum WorkingIndicator {
    _Yes,
    _No,
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
    _Nomatchforinquiry,
    _Nomarketforinstrument,
    _Noinventory,
    _Pass,
    _Other,
}
impl Field for SideComplianceID {
    fn tag(&self) -> u16 {
        return 659;
    }
}
pub struct SideComplianceID {
    value: String,
}
impl Field for AcctIDSource {
    fn tag(&self) -> u16 {
        return 660;
    }
}
#[derive(Debug)]
pub enum AcctIDSource {
    _Bic,
    _Sidcode,
    _Tfm,
    _Omgeo,
    _Dtcccode,
    _Other,
}
impl Field for AllocAcctIDSource {
    fn tag(&self) -> u16 {
        return 661;
    }
}
pub struct AllocAcctIDSource {
    value: u16,
}
impl Field for BenchmarkPrice {
    fn tag(&self) -> u16 {
        return 662;
    }
}
pub struct BenchmarkPrice {
    value: f32,
}
impl Field for BenchmarkPriceType {
    fn tag(&self) -> u16 {
        return 663;
    }
}
pub struct BenchmarkPriceType {
    value: u16,
}
impl Field for ConfirmID {
    fn tag(&self) -> u16 {
        return 664;
    }
}
pub struct ConfirmID {
    value: String,
}
impl Field for ConfirmStatus {
    fn tag(&self) -> u16 {
        return 665;
    }
}
#[derive(Debug)]
pub enum ConfirmStatus {
    _Received,
    _Mismatchedaccount,
    _Missingsettlementinstructions,
    _Confirmed,
    _Requestrejected,
}
impl Field for ConfirmTransType {
    fn tag(&self) -> u16 {
        return 666;
    }
}
#[derive(Debug)]
pub enum ConfirmTransType {
    _New,
    _Replace,
    _Cancel,
}
impl Field for ContractSettlMonth {
    fn tag(&self) -> u16 {
        return 667;
    }
}
pub struct ContractSettlMonth {
    value: u8,
}
impl Field for DeliveryForm {
    fn tag(&self) -> u16 {
        return 668;
    }
}
#[derive(Debug)]
pub enum DeliveryForm {
    _Bookentry,
    _Bearer,
}
impl Field for LastParPx {
    fn tag(&self) -> u16 {
        return 669;
    }
}
pub struct LastParPx {
    value: f32,
}
impl Field for NoLegAllocs {
    fn tag(&self) -> u16 {
        return 670;
    }
}
pub struct NoLegAllocs {
    value: u16,
}
impl Field for LegAllocAccount {
    fn tag(&self) -> u16 {
        return 671;
    }
}
pub struct LegAllocAccount {
    value: String,
}
impl Field for LegIndividualAllocID {
    fn tag(&self) -> u16 {
        return 672;
    }
}
pub struct LegIndividualAllocID {
    value: String,
}
impl Field for LegAllocQty {
    fn tag(&self) -> u16 {
        return 673;
    }
}
pub struct LegAllocQty {
    value: f32,
}
impl Field for LegAllocAcctIDSource {
    fn tag(&self) -> u16 {
        return 674;
    }
}
pub struct LegAllocAcctIDSource {
    value: String,
}
impl Field for LegSettlCurrency {
    fn tag(&self) -> u16 {
        return 675;
    }
}
pub struct LegSettlCurrency {
    value: f32,
}
impl Field for LegBenchmarkCurveCurrency {
    fn tag(&self) -> u16 {
        return 676;
    }
}
pub struct LegBenchmarkCurveCurrency {
    value: f32,
}
impl Field for LegBenchmarkCurveName {
    fn tag(&self) -> u16 {
        return 677;
    }
}
pub struct LegBenchmarkCurveName {
    value: String,
}
impl Field for LegBenchmarkCurvePoint {
    fn tag(&self) -> u16 {
        return 678;
    }
}
pub struct LegBenchmarkCurvePoint {
    value: String,
}
impl Field for LegBenchmarkPrice {
    fn tag(&self) -> u16 {
        return 679;
    }
}
pub struct LegBenchmarkPrice {
    value: f32,
}
impl Field for LegBenchmarkPriceType {
    fn tag(&self) -> u16 {
        return 680;
    }
}
pub struct LegBenchmarkPriceType {
    value: u16,
}
impl Field for LegBidPx {
    fn tag(&self) -> u16 {
        return 681;
    }
}
pub struct LegBidPx {
    value: f32,
}
impl Field for LegIOIQty {
    fn tag(&self) -> u16 {
        return 682;
    }
}
pub struct LegIOIQty {
    value: String,
}
impl Field for NoLegStipulations {
    fn tag(&self) -> u16 {
        return 683;
    }
}
pub struct NoLegStipulations {
    value: u16,
}
impl Field for LegOfferPx {
    fn tag(&self) -> u16 {
        return 684;
    }
}
pub struct LegOfferPx {
    value: f32,
}
impl Field for LegPriceType {
    fn tag(&self) -> u16 {
        return 686;
    }
}
pub struct LegPriceType {
    value: u16,
}
impl Field for LegQty {
    fn tag(&self) -> u16 {
        return 687;
    }
}
pub struct LegQty {
    value: f32,
}
impl Field for LegStipulationType {
    fn tag(&self) -> u16 {
        return 688;
    }
}
pub struct LegStipulationType {
    value: String,
}
impl Field for LegStipulationValue {
    fn tag(&self) -> u16 {
        return 689;
    }
}
pub struct LegStipulationValue {
    value: String,
}
impl Field for LegSwapType {
    fn tag(&self) -> u16 {
        return 690;
    }
}
#[derive(Debug)]
pub enum LegSwapType {
    _Parforpar,
    _Modifiedduration,
    _Risk,
    _Proceeds,
}
impl Field for Pool {
    fn tag(&self) -> u16 {
        return 691;
    }
}
pub struct Pool {
    value: String,
}
impl Field for QuotePriceType {
    fn tag(&self) -> u16 {
        return 692;
    }
}
#[derive(Debug)]
pub enum QuotePriceType {
    _Percent,
    _Pershare,
    _Fixedamount,
    _Discountpercentagepointsbelowpar,
    _Premiumpercentagepointsoverpar,
    _Basispointsrelativetobenchmark,
    _Tedprice,
    _Tedyield,
    _Yieldspread,
    _Yield,
}
impl Field for QuoteRespID {
    fn tag(&self) -> u16 {
        return 693;
    }
}
pub struct QuoteRespID {
    value: String,
}
impl Field for QuoteRespType {
    fn tag(&self) -> u16 {
        return 694;
    }
}
#[derive(Debug)]
pub enum QuoteRespType {
    _Hitlift,
    _Counter,
    _Expired,
    _Cover,
    _Doneaway,
    _Pass,
}
impl Field for QuoteQualifier {
    fn tag(&self) -> u16 {
        return 695;
    }
}
pub struct QuoteQualifier {
    value: char,
}
impl Field for YieldRedemptionDate {
    fn tag(&self) -> u16 {
        return 696;
    }
}
pub struct YieldRedemptionDate {
    value: u64,
}
impl Field for YieldRedemptionPrice {
    fn tag(&self) -> u16 {
        return 697;
    }
}
pub struct YieldRedemptionPrice {
    value: f32,
}
impl Field for YieldRedemptionPriceType {
    fn tag(&self) -> u16 {
        return 698;
    }
}
pub struct YieldRedemptionPriceType {
    value: u16,
}
impl Field for BenchmarkSecurityID {
    fn tag(&self) -> u16 {
        return 699;
    }
}
pub struct BenchmarkSecurityID {
    value: String,
}
impl Field for ReversalIndicator {
    fn tag(&self) -> u16 {
        return 700;
    }
}
pub struct ReversalIndicator {
    value: bool,
}
impl Field for YieldCalcDate {
    fn tag(&self) -> u16 {
        return 701;
    }
}
pub struct YieldCalcDate {
    value: u64,
}
impl Field for NoPositions {
    fn tag(&self) -> u16 {
        return 702;
    }
}
pub struct NoPositions {
    value: u16,
}
impl Field for PosType {
    fn tag(&self) -> u16 {
        return 703;
    }
}
#[derive(Debug)]
pub enum PosType {
    _Transactionquantity,
    _Intraspreadqty,
    _Interspreadqty,
    _Endofdayqty,
    _Startofdayqty,
    _Optionexerciseqty,
    _Optionassignment,
    _Transactionfromexercise,
    _Transactionfromassignment,
    _Pittradeqty,
    _Transfertradeqty,
    _Electronictradeqty,
    _Allocationtradeqty,
    _Adjustmentqty,
    _Asoftradeqty,
    _Deliveryqty,
    _Totaltransactionqty,
    _Crossmarginqty,
    _Integralsplit,
}
impl Field for LongQty {
    fn tag(&self) -> u16 {
        return 704;
    }
}
pub struct LongQty {
    value: f32,
}
impl Field for ShortQty {
    fn tag(&self) -> u16 {
        return 705;
    }
}
pub struct ShortQty {
    value: f32,
}
impl Field for PosQtyStatus {
    fn tag(&self) -> u16 {
        return 706;
    }
}
#[derive(Debug)]
pub enum PosQtyStatus {
    _Submitted,
    _Accepted,
    _Rejected,
}
impl Field for PosAmtType {
    fn tag(&self) -> u16 {
        return 707;
    }
}
#[derive(Debug)]
pub enum PosAmtType {
    _Finalmarktomarketamount,
    _Incrementalmarktomarketamount,
    _Tradevariationamount,
    _Startofdaymarktomarketamount,
    _Premiumamount,
    _Cashresidualamount,
    _Cashamount,
    _Valueadjustedamount,
}
impl Field for PosAmt {
    fn tag(&self) -> u16 {
        return 708;
    }
}
pub struct PosAmt {
    value: f32,
}
impl Field for PosTransType {
    fn tag(&self) -> u16 {
        return 709;
    }
}
#[derive(Debug)]
pub enum PosTransType {
    _Exercise,
    _Donotexercise,
    _Positionadjustment,
    _Positionchangesubmissionmargindisposition,
    _Pledge,
}
impl Field for PosReqID {
    fn tag(&self) -> u16 {
        return 710;
    }
}
pub struct PosReqID {
    value: String,
}
impl Field for NoUnderlyings {
    fn tag(&self) -> u16 {
        return 711;
    }
}
pub struct NoUnderlyings {
    value: u16,
}
impl Field for PosMaintAction {
    fn tag(&self) -> u16 {
        return 712;
    }
}
#[derive(Debug)]
pub enum PosMaintAction {
    _Newusedtoincrementtheoveralltransactionquantity,
    _Replaceusedtooverridetheoveralltransactionquantityorspecificaddmessagesbasedonthereferenceid,
    _Cancelusedtoremovetheoveralltransactionorspecificaddmessagesbasedonreferenceid,
}
impl Field for OrigPosReqRefID {
    fn tag(&self) -> u16 {
        return 713;
    }
}
pub struct OrigPosReqRefID {
    value: String,
}
impl Field for PosMaintRptRefID {
    fn tag(&self) -> u16 {
        return 714;
    }
}
pub struct PosMaintRptRefID {
    value: String,
}
impl Field for ClearingBusinessDate {
    fn tag(&self) -> u16 {
        return 715;
    }
}
pub struct ClearingBusinessDate {
    value: u64,
}
impl Field for SettlSessID {
    fn tag(&self) -> u16 {
        return 716;
    }
}
#[derive(Debug)]
pub enum SettlSessID {
    _Intraday,
    _Regulartradinghours,
    _Electronictradinghours,
}
impl Field for SettlSessSubID {
    fn tag(&self) -> u16 {
        return 717;
    }
}
pub struct SettlSessSubID {
    value: String,
}
impl Field for AdjustmentType {
    fn tag(&self) -> u16 {
        return 718;
    }
}
#[derive(Debug)]
pub enum AdjustmentType {
    _Processrequestasmargindisposition,
    _Deltaplus,
    _Deltaminus,
    _Final,
}
impl Field for ContraryInstructionIndicator {
    fn tag(&self) -> u16 {
        return 719;
    }
}
pub struct ContraryInstructionIndicator {
    value: bool,
}
impl Field for PriorSpreadIndicator {
    fn tag(&self) -> u16 {
        return 720;
    }
}
pub struct PriorSpreadIndicator {
    value: bool,
}
impl Field for PosMaintRptID {
    fn tag(&self) -> u16 {
        return 721;
    }
}
pub struct PosMaintRptID {
    value: String,
}
impl Field for PosMaintStatus {
    fn tag(&self) -> u16 {
        return 722;
    }
}
#[derive(Debug)]
pub enum PosMaintStatus {
    _Accepted,
    _Acceptedwithwarnings,
    _Rejected,
    _Completed,
    _Completedwithwarnings,
}
impl Field for PosMaintResult {
    fn tag(&self) -> u16 {
        return 723;
    }
}
#[derive(Debug)]
pub enum PosMaintResult {
    _Successfulcompletion,
    _Rejected,
    _Other,
}
impl Field for PosReqType {
    fn tag(&self) -> u16 {
        return 724;
    }
}
#[derive(Debug)]
pub enum PosReqType {
    _Positions,
    _Trades,
    _Exercises,
    _Assignments,
}
impl Field for ResponseTransportType {
    fn tag(&self) -> u16 {
        return 725;
    }
}
#[derive(Debug)]
pub enum ResponseTransportType {
    _Inbandtransporttherequestwassentover,
    _Outofbandprearrangedoutofbanddeliverymechanism,
}
impl Field for ResponseDestination {
    fn tag(&self) -> u16 {
        return 726;
    }
}
pub struct ResponseDestination {
    value: String,
}
impl Field for TotalNumPosReports {
    fn tag(&self) -> u16 {
        return 727;
    }
}
pub struct TotalNumPosReports {
    value: u16,
}
impl Field for PosReqResult {
    fn tag(&self) -> u16 {
        return 728;
    }
}
#[derive(Debug)]
pub enum PosReqResult {
    _Validrequest,
    _Invalidorunsupportedrequest,
    _Nopositionsfoundthatmatchcriteria,
    _Notauthorizedtorequestpositions,
    _Requestforpositionnotsupported,
    _Other,
}
impl Field for PosReqStatus {
    fn tag(&self) -> u16 {
        return 729;
    }
}
#[derive(Debug)]
pub enum PosReqStatus {
    _Completed,
    _Completedwithwarnings,
    _Rejected,
}
impl Field for SettlPrice {
    fn tag(&self) -> u16 {
        return 730;
    }
}
pub struct SettlPrice {
    value: f32,
}
impl Field for SettlPriceType {
    fn tag(&self) -> u16 {
        return 731;
    }
}
#[derive(Debug)]
pub enum SettlPriceType {
    _Final,
    _Theoretical,
}
impl Field for UnderlyingSettlPrice {
    fn tag(&self) -> u16 {
        return 732;
    }
}
pub struct UnderlyingSettlPrice {
    value: f32,
}
impl Field for UnderlyingSettlPriceType {
    fn tag(&self) -> u16 {
        return 733;
    }
}
pub struct UnderlyingSettlPriceType {
    value: u16,
}
impl Field for PriorSettlPrice {
    fn tag(&self) -> u16 {
        return 734;
    }
}
pub struct PriorSettlPrice {
    value: f32,
}
impl Field for NoQuoteQualifiers {
    fn tag(&self) -> u16 {
        return 735;
    }
}
pub struct NoQuoteQualifiers {
    value: u16,
}
impl Field for AllocSettlCurrency {
    fn tag(&self) -> u16 {
        return 736;
    }
}
pub struct AllocSettlCurrency {
    value: f32,
}
impl Field for AllocSettlCurrAmt {
    fn tag(&self) -> u16 {
        return 737;
    }
}
pub struct AllocSettlCurrAmt {
    value: f32,
}
impl Field for InterestAtMaturity {
    fn tag(&self) -> u16 {
        return 738;
    }
}
pub struct InterestAtMaturity {
    value: f32,
}
impl Field for LegDatedDate {
    fn tag(&self) -> u16 {
        return 739;
    }
}
pub struct LegDatedDate {
    value: u64,
}
impl Field for LegPool {
    fn tag(&self) -> u16 {
        return 740;
    }
}
pub struct LegPool {
    value: String,
}
impl Field for AllocInterestAtMaturity {
    fn tag(&self) -> u16 {
        return 741;
    }
}
pub struct AllocInterestAtMaturity {
    value: f32,
}
impl Field for AllocAccruedInterestAmt {
    fn tag(&self) -> u16 {
        return 742;
    }
}
pub struct AllocAccruedInterestAmt {
    value: f32,
}
impl Field for DeliveryDate {
    fn tag(&self) -> u16 {
        return 743;
    }
}
pub struct DeliveryDate {
    value: u64,
}
impl Field for AssignmentMethod {
    fn tag(&self) -> u16 {
        return 744;
    }
}
#[derive(Debug)]
pub enum AssignmentMethod {
    _Random,
    _Prorata,
}
impl Field for AssignmentUnit {
    fn tag(&self) -> u16 {
        return 745;
    }
}
pub struct AssignmentUnit {
    value: f32,
}
impl Field for OpenInterest {
    fn tag(&self) -> u16 {
        return 746;
    }
}
pub struct OpenInterest {
    value: f32,
}
impl Field for ExerciseMethod {
    fn tag(&self) -> u16 {
        return 747;
    }
}
#[derive(Debug)]
pub enum ExerciseMethod {
    _Automatic,
    _Manual,
}
impl Field for TotNumTradeReports {
    fn tag(&self) -> u16 {
        return 748;
    }
}
pub struct TotNumTradeReports {
    value: u16,
}
impl Field for TradeRequestResult {
    fn tag(&self) -> u16 {
        return 749;
    }
}
#[derive(Debug)]
pub enum TradeRequestResult {
    _Successful,
    _Invalidorunknowninstrument,
    _Invalidtypeoftraderequested,
    _Invalidparties,
    _Invalidtransporttyperequested,
    _Invaliddestinationrequested,
    _Traderequesttypenotsupported,
    _Unauthorizedfortradecapturereportrequest,
    _Other,
}
impl Field for TradeRequestStatus {
    fn tag(&self) -> u16 {
        return 750;
    }
}
#[derive(Debug)]
pub enum TradeRequestStatus {
    _Accepted,
    _Completed,
    _Rejected,
}
impl Field for TradeReportRejectReason {
    fn tag(&self) -> u16 {
        return 751;
    }
}
#[derive(Debug)]
pub enum TradeReportRejectReason {
    _Successful,
    _Invalidpartyinformation,
    _Unknowninstrument,
    _Unauthorizedtoreporttrades,
    _Invalidtradetype,
    _Other,
}
impl Field for SideMultiLegReportingType {
    fn tag(&self) -> u16 {
        return 752;
    }
}
#[derive(Debug)]
pub enum SideMultiLegReportingType {
    _Singlesecurity,
    _Individuallegofamultilegsecurity,
    _Multilegsecurity,
}
impl Field for NoPosAmt {
    fn tag(&self) -> u16 {
        return 753;
    }
}
pub struct NoPosAmt {
    value: u16,
}
impl Field for AutoAcceptIndicator {
    fn tag(&self) -> u16 {
        return 754;
    }
}
pub struct AutoAcceptIndicator {
    value: bool,
}
impl Field for AllocReportID {
    fn tag(&self) -> u16 {
        return 755;
    }
}
pub struct AllocReportID {
    value: String,
}
impl Field for NoNested2PartyIDs {
    fn tag(&self) -> u16 {
        return 756;
    }
}
pub struct NoNested2PartyIDs {
    value: u16,
}
impl Field for Nested2PartyID {
    fn tag(&self) -> u16 {
        return 757;
    }
}
pub struct Nested2PartyID {
    value: String,
}
impl Field for Nested2PartyIDSource {
    fn tag(&self) -> u16 {
        return 758;
    }
}
pub struct Nested2PartyIDSource {
    value: char,
}
impl Field for Nested2PartyRole {
    fn tag(&self) -> u16 {
        return 759;
    }
}
pub struct Nested2PartyRole {
    value: u16,
}
impl Field for Nested2PartySubID {
    fn tag(&self) -> u16 {
        return 760;
    }
}
pub struct Nested2PartySubID {
    value: String,
}
impl Field for BenchmarkSecurityIDSource {
    fn tag(&self) -> u16 {
        return 761;
    }
}
pub struct BenchmarkSecurityIDSource {
    value: String,
}
impl Field for SecuritySubType {
    fn tag(&self) -> u16 {
        return 762;
    }
}
pub struct SecuritySubType {
    value: String,
}
impl Field for UnderlyingSecuritySubType {
    fn tag(&self) -> u16 {
        return 763;
    }
}
pub struct UnderlyingSecuritySubType {
    value: String,
}
impl Field for LegSecuritySubType {
    fn tag(&self) -> u16 {
        return 764;
    }
}
pub struct LegSecuritySubType {
    value: String,
}
impl Field for AllowableOneSidednessPct {
    fn tag(&self) -> u16 {
        return 765;
    }
}
pub struct AllowableOneSidednessPct {
    value: f32,
}
impl Field for AllowableOneSidednessValue {
    fn tag(&self) -> u16 {
        return 766;
    }
}
pub struct AllowableOneSidednessValue {
    value: f32,
}
impl Field for AllowableOneSidednessCurr {
    fn tag(&self) -> u16 {
        return 767;
    }
}
pub struct AllowableOneSidednessCurr {
    value: f32,
}
impl Field for NoTrdRegTimestamps {
    fn tag(&self) -> u16 {
        return 768;
    }
}
pub struct NoTrdRegTimestamps {
    value: u16,
}
impl Field for TrdRegTimestamp {
    fn tag(&self) -> u16 {
        return 769;
    }
}
pub struct TrdRegTimestamp {
    value: u64,
}
impl Field for TrdRegTimestampType {
    fn tag(&self) -> u16 {
        return 770;
    }
}
#[derive(Debug)]
pub enum TrdRegTimestampType {
    _Executiontime,
    _Timein,
    _Timeout,
    _Brokerreceipt,
    _Brokerexecution,
}
impl Field for TrdRegTimestampOrigin {
    fn tag(&self) -> u16 {
        return 771;
    }
}
pub struct TrdRegTimestampOrigin {
    value: String,
}
impl Field for ConfirmRefID {
    fn tag(&self) -> u16 {
        return 772;
    }
}
pub struct ConfirmRefID {
    value: String,
}
impl Field for ConfirmType {
    fn tag(&self) -> u16 {
        return 773;
    }
}
#[derive(Debug)]
pub enum ConfirmType {
    _Status,
    _Confirmation,
    _Confirmationrequestrejected,
}
impl Field for ConfirmRejReason {
    fn tag(&self) -> u16 {
        return 774;
    }
}
#[derive(Debug)]
pub enum ConfirmRejReason {
    _Mismatchedaccount,
    _Missingsettlementinstructions,
    _Other,
}
impl Field for BookingType {
    fn tag(&self) -> u16 {
        return 775;
    }
}
#[derive(Debug)]
pub enum BookingType {
    _Regularbooking,
    _Cfd,
    _Totalreturnswap,
}
impl Field for IndividualAllocRejCode {
    fn tag(&self) -> u16 {
        return 776;
    }
}
pub struct IndividualAllocRejCode {
    value: u16,
}
impl Field for SettlInstMsgID {
    fn tag(&self) -> u16 {
        return 777;
    }
}
pub struct SettlInstMsgID {
    value: String,
}
impl Field for NoSettlInst {
    fn tag(&self) -> u16 {
        return 778;
    }
}
pub struct NoSettlInst {
    value: u16,
}
impl Field for LastUpdateTime {
    fn tag(&self) -> u16 {
        return 779;
    }
}
pub struct LastUpdateTime {
    value: u64,
}
impl Field for AllocSettlInstType {
    fn tag(&self) -> u16 {
        return 780;
    }
}
#[derive(Debug)]
pub enum AllocSettlInstType {
    _Usedefaultinstructions,
    _Derivefromparametersprovided,
    _Fulldetailsprovided,
    _Ssidbidsprovided,
    _Phoneforinstructions,
}
impl Field for NoSettlPartyIDs {
    fn tag(&self) -> u16 {
        return 781;
    }
}
pub struct NoSettlPartyIDs {
    value: u16,
}
impl Field for SettlPartyID {
    fn tag(&self) -> u16 {
        return 782;
    }
}
pub struct SettlPartyID {
    value: String,
}
impl Field for SettlPartyIDSource {
    fn tag(&self) -> u16 {
        return 783;
    }
}
pub struct SettlPartyIDSource {
    value: char,
}
impl Field for SettlPartyRole {
    fn tag(&self) -> u16 {
        return 784;
    }
}
pub struct SettlPartyRole {
    value: u16,
}
impl Field for SettlPartySubID {
    fn tag(&self) -> u16 {
        return 785;
    }
}
pub struct SettlPartySubID {
    value: String,
}
impl Field for SettlPartySubIDType {
    fn tag(&self) -> u16 {
        return 786;
    }
}
pub struct SettlPartySubIDType {
    value: u16,
}
impl Field for DlvyInstType {
    fn tag(&self) -> u16 {
        return 787;
    }
}
#[derive(Debug)]
pub enum DlvyInstType {
    _Securities,
    _Cash,
}
impl Field for TerminationType {
    fn tag(&self) -> u16 {
        return 788;
    }
}
#[derive(Debug)]
pub enum TerminationType {
    _Overnight,
    _Term,
    _Flexible,
    _Open,
}
impl Field for NextExpectedMsgSeqNum {
    fn tag(&self) -> u16 {
        return 789;
    }
}
pub struct NextExpectedMsgSeqNum {
    value: u64,
}
impl Field for OrdStatusReqID {
    fn tag(&self) -> u16 {
        return 790;
    }
}
pub struct OrdStatusReqID {
    value: String,
}
impl Field for SettlInstReqID {
    fn tag(&self) -> u16 {
        return 791;
    }
}
pub struct SettlInstReqID {
    value: String,
}
impl Field for SettlInstReqRejCode {
    fn tag(&self) -> u16 {
        return 792;
    }
}
#[derive(Debug)]
pub enum SettlInstReqRejCode {
    _Unabletoprocessrequest,
    _Unknownaccount,
    _Nomatchingsettlementinstructionsfound,
    _Other,
}
impl Field for SecondaryAllocID {
    fn tag(&self) -> u16 {
        return 793;
    }
}
pub struct SecondaryAllocID {
    value: String,
}
impl Field for AllocReportType {
    fn tag(&self) -> u16 {
        return 794;
    }
}
#[derive(Debug)]
pub enum AllocReportType {
    _Sellsidecalculatedusingpreliminary,
    _Sellsidecalculatedwithoutpreliminary,
    _Warehouserecap,
    _Requesttointermediary,
}
impl Field for AllocReportRefID {
    fn tag(&self) -> u16 {
        return 795;
    }
}
pub struct AllocReportRefID {
    value: String,
}
impl Field for AllocCancReplaceReason {
    fn tag(&self) -> u16 {
        return 796;
    }
}
#[derive(Debug)]
pub enum AllocCancReplaceReason {
    _Originaldetailsincompleteincorrect,
    _Changeinunderlyingorderdetails,
    _Other,
}
impl Field for CopyMsgIndicator {
    fn tag(&self) -> u16 {
        return 797;
    }
}
pub struct CopyMsgIndicator {
    value: bool,
}
impl Field for AllocAccountType {
    fn tag(&self) -> u16 {
        return 798;
    }
}
#[derive(Debug)]
pub enum AllocAccountType {
    _Accountiscarriedoncustomersideofbooks,
    _Accountiscarriedonnoncustomersideofbooks,
    _Housetrader,
    _Floortrader,
    _Accountiscarriedonnoncustomersideofbooksandiscrossmargined,
    _Accountishousetraderandiscrossmargined,
    _Jointbackofficeaccount,
}
impl Field for OrderAvgPx {
    fn tag(&self) -> u16 {
        return 799;
    }
}
pub struct OrderAvgPx {
    value: f32,
}
impl Field for OrderBookingQty {
    fn tag(&self) -> u16 {
        return 800;
    }
}
pub struct OrderBookingQty {
    value: f32,
}
impl Field for NoSettlPartySubIDs {
    fn tag(&self) -> u16 {
        return 801;
    }
}
pub struct NoSettlPartySubIDs {
    value: u16,
}
impl Field for NoPartySubIDs {
    fn tag(&self) -> u16 {
        return 802;
    }
}
pub struct NoPartySubIDs {
    value: u16,
}
impl Field for PartySubIDType {
    fn tag(&self) -> u16 {
        return 803;
    }
}
#[derive(Debug)]
pub enum PartySubIDType {
    _Firm,
    _Person,
    _System,
    _Application,
    _Fulllegalnameoffirm,
    _Postaladdress,
    _Phonenumber,
    _Emailaddress,
    _Contactname,
    _Securitiesaccountnumber,
    _Registrationnumber,
    _Registeredaddress12,
    _Regulatorystatus,
    _Registrationname,
    _Cashaccountnumber,
    _Bic,
    _Csdparticipantmembercode,
    _Registeredaddress18,
    _Fundaccountname,
    _Telexnumber,
    _Faxnumber,
    _Securitiesaccountname,
    _Cashaccountname,
    _Department,
    _Location,
    _Positionaccounttype,
}
impl Field for NoNestedPartySubIDs {
    fn tag(&self) -> u16 {
        return 804;
    }
}
pub struct NoNestedPartySubIDs {
    value: u16,
}
impl Field for NestedPartySubIDType {
    fn tag(&self) -> u16 {
        return 805;
    }
}
pub struct NestedPartySubIDType {
    value: u16,
}
impl Field for NoNested2PartySubIDs {
    fn tag(&self) -> u16 {
        return 806;
    }
}
pub struct NoNested2PartySubIDs {
    value: u16,
}
impl Field for Nested2PartySubIDType {
    fn tag(&self) -> u16 {
        return 807;
    }
}
pub struct Nested2PartySubIDType {
    value: u16,
}
impl Field for AllocIntermedReqType {
    fn tag(&self) -> u16 {
        return 808;
    }
}
#[derive(Debug)]
pub enum AllocIntermedReqType {
    _Pendingaccept,
    _Pendingrelease,
    _Pendingreversal,
    _Accept,
    _Blocklevelreject,
    _Accountlevelreject,
}
impl Field for UnderlyingPx {
    fn tag(&self) -> u16 {
        return 810;
    }
}
pub struct UnderlyingPx {
    value: f32,
}
impl Field for PriceDelta {
    fn tag(&self) -> u16 {
        return 811;
    }
}
pub struct PriceDelta {
    value: f32,
}
impl Field for ApplQueueMax {
    fn tag(&self) -> u16 {
        return 812;
    }
}
pub struct ApplQueueMax {
    value: u16,
}
impl Field for ApplQueueDepth {
    fn tag(&self) -> u16 {
        return 813;
    }
}
pub struct ApplQueueDepth {
    value: u16,
}
impl Field for ApplQueueResolution {
    fn tag(&self) -> u16 {
        return 814;
    }
}
#[derive(Debug)]
pub enum ApplQueueResolution {
    _Noactiontaken,
    _Queueflushed,
    _Overlaylast,
    _Endsession,
}
impl Field for ApplQueueAction {
    fn tag(&self) -> u16 {
        return 815;
    }
}
#[derive(Debug)]
pub enum ApplQueueAction {
    _Noactiontaken,
    _Queueflushed,
    _Overlaylast,
    _Endsession,
}
impl Field for NoAltMDSource {
    fn tag(&self) -> u16 {
        return 816;
    }
}
pub struct NoAltMDSource {
    value: u16,
}
impl Field for AltMDSourceID {
    fn tag(&self) -> u16 {
        return 817;
    }
}
pub struct AltMDSourceID {
    value: String,
}
impl Field for SecondaryTradeReportID {
    fn tag(&self) -> u16 {
        return 818;
    }
}
pub struct SecondaryTradeReportID {
    value: String,
}
impl Field for AvgPxIndicator {
    fn tag(&self) -> u16 {
        return 819;
    }
}
#[derive(Debug)]
pub enum AvgPxIndicator {
    _Noaveragepricing,
    _Tradeispartofanaveragepricegroupidentifiedbythetradelinkid,
    _Lasttradeintheaveragepricegroupidentifiedbythetradelinkid,
}
impl Field for TradeLinkID {
    fn tag(&self) -> u16 {
        return 820;
    }
}
pub struct TradeLinkID {
    value: String,
}
impl Field for OrderInputDevice {
    fn tag(&self) -> u16 {
        return 821;
    }
}
pub struct OrderInputDevice {
    value: String,
}
impl Field for UnderlyingTradingSessionID {
    fn tag(&self) -> u16 {
        return 822;
    }
}
pub struct UnderlyingTradingSessionID {
    value: String,
}
impl Field for UnderlyingTradingSessionSubID {
    fn tag(&self) -> u16 {
        return 823;
    }
}
pub struct UnderlyingTradingSessionSubID {
    value: String,
}
impl Field for TradeLegRefID {
    fn tag(&self) -> u16 {
        return 824;
    }
}
pub struct TradeLegRefID {
    value: String,
}
impl Field for ExchangeRule {
    fn tag(&self) -> u16 {
        return 825;
    }
}
pub struct ExchangeRule {
    value: String,
}
impl Field for TradeAllocIndicator {
    fn tag(&self) -> u16 {
        return 826;
    }
}
#[derive(Debug)]
pub enum TradeAllocIndicator {
    _Allocationnotrequired,
    _Allocationrequired,
    _Useallocationprovidedwiththetrade,
}
impl Field for ExpirationCycle {
    fn tag(&self) -> u16 {
        return 827;
    }
}
#[derive(Debug)]
pub enum ExpirationCycle {
    _Expireontradingsessionclose,
    _Expireontradingsessionopen,
}
impl Field for TrdType {
    fn tag(&self) -> u16 {
        return 828;
    }
}
#[derive(Debug)]
pub enum TrdType {
    _Regulartrade,
    _Blocktrade,
    _Efp,
    _Transfer,
    _Latetrade,
    _Ttrade,
    _Weightedaveragepricetrade,
    _Bunchedtrade,
    _Latebunchedtrade,
    _Priorreferencepricetrade,
    _Afterhourstrade,
}
impl Field for TrdSubType {
    fn tag(&self) -> u16 {
        return 829;
    }
}
pub struct TrdSubType {
    value: u16,
}
impl Field for TransferReason {
    fn tag(&self) -> u16 {
        return 830;
    }
}
pub struct TransferReason {
    value: String,
}
impl Field for TotNumAssignmentReports {
    fn tag(&self) -> u16 {
        return 832;
    }
}
pub struct TotNumAssignmentReports {
    value: u16,
}
impl Field for AsgnRptID {
    fn tag(&self) -> u16 {
        return 833;
    }
}
pub struct AsgnRptID {
    value: String,
}
impl Field for ThresholdAmount {
    fn tag(&self) -> u16 {
        return 834;
    }
}
pub struct ThresholdAmount {
    value: i8,
}
impl Field for PegMoveType {
    fn tag(&self) -> u16 {
        return 835;
    }
}
#[derive(Debug)]
pub enum PegMoveType {
    _Floating,
    _Fixed,
}
impl Field for PegOffsetType {
    fn tag(&self) -> u16 {
        return 836;
    }
}
#[derive(Debug)]
pub enum PegOffsetType {
    _Price,
    _Basispoints,
    _Ticks,
    _Pricetier,
}
impl Field for PegLimitType {
    fn tag(&self) -> u16 {
        return 837;
    }
}
#[derive(Debug)]
pub enum PegLimitType {
    _Orbetter,
    _Strictlimitisastrictlimit,
    _Orworseforabuythepeglimitisaminimumandforasellthepeglimitisamaximum,
}
impl Field for PegRoundDirection {
    fn tag(&self) -> u16 {
        return 838;
    }
}
#[derive(Debug)]
pub enum PegRoundDirection {
    _Moreaggressiveonabuyorderroundthepriceuprounduptothenearesttickonasellrounddowntothenearesttick,
    _Morepassiveonabuyorderrounddowntonearesttickonasellorderrounduptonearesttick,
}
impl Field for PeggedPrice {
    fn tag(&self) -> u16 {
        return 839;
    }
}
pub struct PeggedPrice {
    value: f32,
}
impl Field for PegScope {
    fn tag(&self) -> u16 {
        return 840;
    }
}
#[derive(Debug)]
pub enum PegScope {
    _Local,
    _National,
    _Global,
    _Nationalexcludinglocal,
}
impl Field for DiscretionMoveType {
    fn tag(&self) -> u16 {
        return 841;
    }
}
#[derive(Debug)]
pub enum DiscretionMoveType {
    _Floating,
    _Fixed,
}
impl Field for DiscretionOffsetType {
    fn tag(&self) -> u16 {
        return 842;
    }
}
#[derive(Debug)]
pub enum DiscretionOffsetType {
    _Price,
    _Basispoints,
    _Ticks,
    _Pricetier,
}
impl Field for DiscretionLimitType {
    fn tag(&self) -> u16 {
        return 843;
    }
}
#[derive(Debug)]
pub enum DiscretionLimitType {
    _Orbetter,
    _Strictlimitisastrictlimit,
    _Orworseforabuythediscretionpriceisaminimumandforasellthediscretionpriceisamaximum,
}
impl Field for DiscretionRoundDirection {
    fn tag(&self) -> u16 {
        return 844;
    }
}
#[derive(Debug)]
pub enum DiscretionRoundDirection {
    _Moreaggressiveonabuyorderroundthepriceuprounduptothenearesttickonasellrounddowntothenearesttick,
    _Morepassiveonabuyorderrounddowntonearesttickonasellorderrounduptonearesttick,
}
impl Field for DiscretionPrice {
    fn tag(&self) -> u16 {
        return 845;
    }
}
pub struct DiscretionPrice {
    value: f32,
}
impl Field for DiscretionScope {
    fn tag(&self) -> u16 {
        return 846;
    }
}
#[derive(Debug)]
pub enum DiscretionScope {
    _Local,
    _National,
    _Global,
    _Nationalexcludinglocal,
}
impl Field for TargetStrategy {
    fn tag(&self) -> u16 {
        return 847;
    }
}
#[derive(Debug)]
pub enum TargetStrategy {
    _Vwap,
    _Participate,
    _Mininizemarketimpact,
}
impl Field for TargetStrategyParameters {
    fn tag(&self) -> u16 {
        return 848;
    }
}
pub struct TargetStrategyParameters {
    value: String,
}
impl Field for ParticipationRate {
    fn tag(&self) -> u16 {
        return 849;
    }
}
pub struct ParticipationRate {
    value: f32,
}
impl Field for TargetStrategyPerformance {
    fn tag(&self) -> u16 {
        return 850;
    }
}
pub struct TargetStrategyPerformance {
    value: f32,
}
impl Field for LastLiquidityInd {
    fn tag(&self) -> u16 {
        return 851;
    }
}
#[derive(Debug)]
pub enum LastLiquidityInd {
    _Addedliquidity,
    _Removedliquidity,
    _Liquidityroutedout,
}
impl Field for PublishTrdIndicator {
    fn tag(&self) -> u16 {
        return 852;
    }
}
#[derive(Debug)]
pub enum PublishTrdIndicator {
    _Yes,
    _No,
}
impl Field for ShortSaleReason {
    fn tag(&self) -> u16 {
        return 853;
    }
}
#[derive(Debug)]
pub enum ShortSaleReason {
    _Dealersoldshort,
    _Dealersoldshortexempt,
    _Sellingcustomersoldshort,
    _Sellingcustomersoldshortexempt,
    _Qualifedservicerepresentative,
    _Qsroragucontrasidesoldshortexempt,
}
impl Field for QtyType {
    fn tag(&self) -> u16 {
        return 854;
    }
}
#[derive(Debug)]
pub enum QtyType {
    _Units,
    _Contracts,
}
impl Field for SecondaryTrdType {
    fn tag(&self) -> u16 {
        return 855;
    }
}
pub struct SecondaryTrdType {
    value: u16,
}
impl Field for TradeReportType {
    fn tag(&self) -> u16 {
        return 856;
    }
}
#[derive(Debug)]
pub enum TradeReportType {
    _Submit,
    _Alleged,
    _Accept,
    _Decline,
    _Addendum,
    _Nowas,
    _Tradereportcancel,
    _Lockedintradebreak,
}
impl Field for AllocNoOrdersType {
    fn tag(&self) -> u16 {
        return 857;
    }
}
#[derive(Debug)]
pub enum AllocNoOrdersType {
    _Notspecified,
    _Explicitlistprovided,
}
impl Field for SharedCommission {
    fn tag(&self) -> u16 {
        return 858;
    }
}
pub struct SharedCommission {
    value: f32,
}
impl Field for ConfirmReqID {
    fn tag(&self) -> u16 {
        return 859;
    }
}
pub struct ConfirmReqID {
    value: String,
}
impl Field for AvgParPx {
    fn tag(&self) -> u16 {
        return 860;
    }
}
pub struct AvgParPx {
    value: f32,
}
impl Field for ReportedPx {
    fn tag(&self) -> u16 {
        return 861;
    }
}
pub struct ReportedPx {
    value: f32,
}
impl Field for NoCapacities {
    fn tag(&self) -> u16 {
        return 862;
    }
}
pub struct NoCapacities {
    value: u16,
}
impl Field for OrderCapacityQty {
    fn tag(&self) -> u16 {
        return 863;
    }
}
pub struct OrderCapacityQty {
    value: f32,
}
impl Field for NoEvents {
    fn tag(&self) -> u16 {
        return 864;
    }
}
pub struct NoEvents {
    value: u16,
}
impl Field for EventType {
    fn tag(&self) -> u16 {
        return 865;
    }
}
#[derive(Debug)]
pub enum EventType {
    _Put,
    _Call,
    _Tender,
    _Sinkingfundcall,
    _Other,
}
impl Field for EventDate {
    fn tag(&self) -> u16 {
        return 866;
    }
}
pub struct EventDate {
    value: u64,
}
impl Field for EventPx {
    fn tag(&self) -> u16 {
        return 867;
    }
}
pub struct EventPx {
    value: f32,
}
impl Field for EventText {
    fn tag(&self) -> u16 {
        return 868;
    }
}
pub struct EventText {
    value: String,
}
impl Field for PctAtRisk {
    fn tag(&self) -> u16 {
        return 869;
    }
}
pub struct PctAtRisk {
    value: f32,
}
impl Field for NoInstrAttrib {
    fn tag(&self) -> u16 {
        return 870;
    }
}
pub struct NoInstrAttrib {
    value: u16,
}
impl Field for InstrAttribType {
    fn tag(&self) -> u16 {
        return 871;
    }
}
#[derive(Debug)]
pub enum InstrAttribType {
    _Flat,
    _Zerocoupon,
    _Interestbearing,
    _Noperiodicpayments,
    _Variablerate,
    _Lessfeeforput,
    _Steppedcoupon,
    _Couponperiod,
    _Whenandifissued,
    _Originalissuediscount,
    _Callableputtable,
    _Escrowedtomaturity,
    _Escrowedtoredemptiondatecallablesupplyredemptiondateintheinstrattribvalue,
    _Prerefunded,
    _Indefault,
    _Unrated,
    _Taxable,
    _Indexed,
    _Subjecttoalternativeminimumtax,
    _Originalissuediscountpricesupplypriceintheinstrattribvalue,
    _Callablebelowmaturityvalue,
    _Callablewithoutnoticebymailtoholderunlessregistered,
    _Textsupplythetextoftheattributeordisclaimerintheinstrattribvalue,
}
impl Field for InstrAttribValue {
    fn tag(&self) -> u16 {
        return 872;
    }
}
pub struct InstrAttribValue {
    value: String,
}
impl Field for DatedDate {
    fn tag(&self) -> u16 {
        return 873;
    }
}
pub struct DatedDate {
    value: u64,
}
impl Field for InterestAccrualDate {
    fn tag(&self) -> u16 {
        return 874;
    }
}
pub struct InterestAccrualDate {
    value: u64,
}
impl Field for CPProgram {
    fn tag(&self) -> u16 {
        return 875;
    }
}
#[derive(Debug)]
pub enum CPProgram {
    _3,
    _4,
    _Other,
}
impl Field for CPRegType {
    fn tag(&self) -> u16 {
        return 876;
    }
}
pub struct CPRegType {
    value: String,
}
impl Field for UnderlyingCPProgram {
    fn tag(&self) -> u16 {
        return 877;
    }
}
pub struct UnderlyingCPProgram {
    value: String,
}
impl Field for UnderlyingCPRegType {
    fn tag(&self) -> u16 {
        return 878;
    }
}
pub struct UnderlyingCPRegType {
    value: String,
}
impl Field for UnderlyingQty {
    fn tag(&self) -> u16 {
        return 879;
    }
}
pub struct UnderlyingQty {
    value: f32,
}
impl Field for TrdMatchID {
    fn tag(&self) -> u16 {
        return 880;
    }
}
pub struct TrdMatchID {
    value: String,
}
impl Field for SecondaryTradeReportRefID {
    fn tag(&self) -> u16 {
        return 881;
    }
}
pub struct SecondaryTradeReportRefID {
    value: String,
}
impl Field for UnderlyingDirtyPrice {
    fn tag(&self) -> u16 {
        return 882;
    }
}
pub struct UnderlyingDirtyPrice {
    value: f32,
}
impl Field for UnderlyingEndPrice {
    fn tag(&self) -> u16 {
        return 883;
    }
}
pub struct UnderlyingEndPrice {
    value: f32,
}
impl Field for UnderlyingStartValue {
    fn tag(&self) -> u16 {
        return 884;
    }
}
pub struct UnderlyingStartValue {
    value: f32,
}
impl Field for UnderlyingCurrentValue {
    fn tag(&self) -> u16 {
        return 885;
    }
}
pub struct UnderlyingCurrentValue {
    value: f32,
}
impl Field for UnderlyingEndValue {
    fn tag(&self) -> u16 {
        return 886;
    }
}
pub struct UnderlyingEndValue {
    value: f32,
}
impl Field for NoUnderlyingStips {
    fn tag(&self) -> u16 {
        return 887;
    }
}
pub struct NoUnderlyingStips {
    value: u16,
}
impl Field for UnderlyingStipType {
    fn tag(&self) -> u16 {
        return 888;
    }
}
pub struct UnderlyingStipType {
    value: String,
}
impl Field for UnderlyingStipValue {
    fn tag(&self) -> u16 {
        return 889;
    }
}
pub struct UnderlyingStipValue {
    value: String,
}
impl Field for MaturityNetMoney {
    fn tag(&self) -> u16 {
        return 890;
    }
}
pub struct MaturityNetMoney {
    value: f32,
}
impl Field for MiscFeeBasis {
    fn tag(&self) -> u16 {
        return 891;
    }
}
#[derive(Debug)]
pub enum MiscFeeBasis {
    _Absolute,
    _Perunit,
    _Percentage,
}
impl Field for TotNoAllocs {
    fn tag(&self) -> u16 {
        return 892;
    }
}
pub struct TotNoAllocs {
    value: u16,
}
impl Field for LastFragment {
    fn tag(&self) -> u16 {
        return 893;
    }
}
#[derive(Debug)]
pub enum LastFragment {
    _Yes,
    _No,
}
impl Field for CollReqID {
    fn tag(&self) -> u16 {
        return 894;
    }
}
pub struct CollReqID {
    value: String,
}
impl Field for CollAsgnReason {
    fn tag(&self) -> u16 {
        return 895;
    }
}
#[derive(Debug)]
pub enum CollAsgnReason {
    _Initial,
    _Scheduled,
    _Timewarning,
    _Margindeficiency,
    _Marginexcess,
    _Forwardcollateraldemand,
    _Eventofdefault,
    _Adversetaxevent,
}
impl Field for CollInquiryQualifier {
    fn tag(&self) -> u16 {
        return 896;
    }
}
#[derive(Debug)]
pub enum CollInquiryQualifier {
    _Tradedate,
    _Gcinstrument,
    _Collateralinstrument,
    _Substitutioneligible,
    _Notassigned,
    _Partiallyassigned,
    _Fullyassigned,
    _Outstandingtrades,
}
impl Field for NoTrades {
    fn tag(&self) -> u16 {
        return 897;
    }
}
pub struct NoTrades {
    value: u16,
}
impl Field for MarginRatio {
    fn tag(&self) -> u16 {
        return 898;
    }
}
pub struct MarginRatio {
    value: f32,
}
impl Field for MarginExcess {
    fn tag(&self) -> u16 {
        return 899;
    }
}
pub struct MarginExcess {
    value: f32,
}
impl Field for TotalNetValue {
    fn tag(&self) -> u16 {
        return 900;
    }
}
pub struct TotalNetValue {
    value: f32,
}
impl Field for CashOutstanding {
    fn tag(&self) -> u16 {
        return 901;
    }
}
pub struct CashOutstanding {
    value: f32,
}
impl Field for CollAsgnID {
    fn tag(&self) -> u16 {
        return 902;
    }
}
pub struct CollAsgnID {
    value: String,
}
impl Field for CollAsgnTransType {
    fn tag(&self) -> u16 {
        return 903;
    }
}
#[derive(Debug)]
pub enum CollAsgnTransType {
    _New,
    _Replace,
    _Cancel,
    _Release,
    _Reverse,
}
impl Field for CollRespID {
    fn tag(&self) -> u16 {
        return 904;
    }
}
pub struct CollRespID {
    value: String,
}
impl Field for CollAsgnRespType {
    fn tag(&self) -> u16 {
        return 905;
    }
}
#[derive(Debug)]
pub enum CollAsgnRespType {
    _Received,
    _Accepted,
    _Declined,
    _Rejected,
}
impl Field for CollAsgnRejectReason {
    fn tag(&self) -> u16 {
        return 906;
    }
}
#[derive(Debug)]
pub enum CollAsgnRejectReason {
    _Unknowndeal,
    _Unknownorinvalidinstrument,
    _Unauthorizedtransaction,
    _Insufficientcollateral,
    _Invalidtypeofcollateral,
    _Excessivesubstitution,
    _Other,
}
impl Field for CollAsgnRefID {
    fn tag(&self) -> u16 {
        return 907;
    }
}
pub struct CollAsgnRefID {
    value: String,
}
impl Field for CollRptID {
    fn tag(&self) -> u16 {
        return 908;
    }
}
pub struct CollRptID {
    value: String,
}
impl Field for CollInquiryID {
    fn tag(&self) -> u16 {
        return 909;
    }
}
pub struct CollInquiryID {
    value: String,
}
impl Field for CollStatus {
    fn tag(&self) -> u16 {
        return 910;
    }
}
#[derive(Debug)]
pub enum CollStatus {
    _Unassigned,
    _Partiallyassigned,
    _Assignmentproposed,
    _Assigned,
    _Challenged,
}
impl Field for TotNumReports {
    fn tag(&self) -> u16 {
        return 911;
    }
}
pub struct TotNumReports {
    value: u16,
}
impl Field for LastRptRequested {
    fn tag(&self) -> u16 {
        return 912;
    }
}
pub struct LastRptRequested {
    value: bool,
}
impl Field for AgreementDesc {
    fn tag(&self) -> u16 {
        return 913;
    }
}
pub struct AgreementDesc {
    value: String,
}
impl Field for AgreementID {
    fn tag(&self) -> u16 {
        return 914;
    }
}
pub struct AgreementID {
    value: String,
}
impl Field for AgreementDate {
    fn tag(&self) -> u16 {
        return 915;
    }
}
pub struct AgreementDate {
    value: u64,
}
impl Field for StartDate {
    fn tag(&self) -> u16 {
        return 916;
    }
}
pub struct StartDate {
    value: u64,
}
impl Field for EndDate {
    fn tag(&self) -> u16 {
        return 917;
    }
}
pub struct EndDate {
    value: u64,
}
impl Field for AgreementCurrency {
    fn tag(&self) -> u16 {
        return 918;
    }
}
pub struct AgreementCurrency {
    value: f32,
}
impl Field for DeliveryType {
    fn tag(&self) -> u16 {
        return 919;
    }
}
#[derive(Debug)]
pub enum DeliveryType {
    _Versuspaymentdeliver,
    _Freedeliver,
    _Triparty,
    _Holdincustody,
}
impl Field for EndAccruedInterestAmt {
    fn tag(&self) -> u16 {
        return 920;
    }
}
pub struct EndAccruedInterestAmt {
    value: f32,
}
impl Field for StartCash {
    fn tag(&self) -> u16 {
        return 921;
    }
}
pub struct StartCash {
    value: f32,
}
impl Field for EndCash {
    fn tag(&self) -> u16 {
        return 922;
    }
}
pub struct EndCash {
    value: f32,
}
impl Field for UserRequestID {
    fn tag(&self) -> u16 {
        return 923;
    }
}
pub struct UserRequestID {
    value: String,
}
impl Field for UserRequestType {
    fn tag(&self) -> u16 {
        return 924;
    }
}
#[derive(Debug)]
pub enum UserRequestType {
    _Logonuser,
    _Logoffuser,
    _Changepasswordforuser,
    _Requestindividualuserstatus,
}
impl Field for NewPassword {
    fn tag(&self) -> u16 {
        return 925;
    }
}
pub struct NewPassword {
    value: String,
}
impl Field for UserStatus {
    fn tag(&self) -> u16 {
        return 926;
    }
}
#[derive(Debug)]
pub enum UserStatus {
    _Loggedin,
    _Notloggedin,
    _Usernotrecognised,
    _Passwordincorrect,
    _Passwordchanged,
    _Other,
}
impl Field for UserStatusText {
    fn tag(&self) -> u16 {
        return 927;
    }
}
pub struct UserStatusText {
    value: String,
}
impl Field for StatusValue {
    fn tag(&self) -> u16 {
        return 928;
    }
}
#[derive(Debug)]
pub enum StatusValue {
    _Connected,
    _Notconnecteddownexpectedup,
    _Notconnecteddownexpecteddown,
    _Inprocess,
}
impl Field for StatusText {
    fn tag(&self) -> u16 {
        return 929;
    }
}
pub struct StatusText {
    value: String,
}
impl Field for RefCompID {
    fn tag(&self) -> u16 {
        return 930;
    }
}
pub struct RefCompID {
    value: String,
}
impl Field for RefSubID {
    fn tag(&self) -> u16 {
        return 931;
    }
}
pub struct RefSubID {
    value: String,
}
impl Field for NetworkResponseID {
    fn tag(&self) -> u16 {
        return 932;
    }
}
pub struct NetworkResponseID {
    value: String,
}
impl Field for NetworkRequestID {
    fn tag(&self) -> u16 {
        return 933;
    }
}
pub struct NetworkRequestID {
    value: String,
}
impl Field for LastNetworkResponseID {
    fn tag(&self) -> u16 {
        return 934;
    }
}
pub struct LastNetworkResponseID {
    value: String,
}
impl Field for NetworkRequestType {
    fn tag(&self) -> u16 {
        return 935;
    }
}
#[derive(Debug)]
pub enum NetworkRequestType {
    _Snapshot,
    _Subscribe,
    _Stopsubscribing,
    _Levelofdetailthennocompidsbecomesrequired,
}
impl Field for NoCompIDs {
    fn tag(&self) -> u16 {
        return 936;
    }
}
pub struct NoCompIDs {
    value: u16,
}
impl Field for NetworkStatusResponseType {
    fn tag(&self) -> u16 {
        return 937;
    }
}
#[derive(Debug)]
pub enum NetworkStatusResponseType {
    _Full,
    _Incrementalupdate,
}
impl Field for NoCollInquiryQualifier {
    fn tag(&self) -> u16 {
        return 938;
    }
}
pub struct NoCollInquiryQualifier {
    value: u16,
}
impl Field for TrdRptStatus {
    fn tag(&self) -> u16 {
        return 939;
    }
}
#[derive(Debug)]
pub enum TrdRptStatus {
    _Accepted,
    _Rejected,
}
impl Field for AffirmStatus {
    fn tag(&self) -> u16 {
        return 940;
    }
}
#[derive(Debug)]
pub enum AffirmStatus {
    _Received,
    _Confirmrejectedienotaffirmed,
    _Affirmed,
}
impl Field for UnderlyingStrikeCurrency {
    fn tag(&self) -> u16 {
        return 941;
    }
}
pub struct UnderlyingStrikeCurrency {
    value: f32,
}
impl Field for LegStrikeCurrency {
    fn tag(&self) -> u16 {
        return 942;
    }
}
pub struct LegStrikeCurrency {
    value: f32,
}
impl Field for TimeBracket {
    fn tag(&self) -> u16 {
        return 943;
    }
}
pub struct TimeBracket {
    value: String,
}
impl Field for CollAction {
    fn tag(&self) -> u16 {
        return 944;
    }
}
#[derive(Debug)]
pub enum CollAction {
    _Retain,
    _Add,
    _Remove,
}
impl Field for CollInquiryStatus {
    fn tag(&self) -> u16 {
        return 945;
    }
}
#[derive(Debug)]
pub enum CollInquiryStatus {
    _Accepted,
    _Acceptedwithwarnings,
    _Completed,
    _Completedwithwarnings,
    _Rejected,
}
impl Field for CollInquiryResult {
    fn tag(&self) -> u16 {
        return 946;
    }
}
#[derive(Debug)]
pub enum CollInquiryResult {
    _Successful,
    _Invalidorunknowninstrument,
    _Invalidorunknowncollateraltype,
    _Invalidparties,
    _Invalidtransporttyperequested,
    _Invaliddestinationrequested,
    _Nocollateralfoundforthetradespecified,
    _Nocollateralfoundfortheorderspecified,
    _Collateralinquirytypenotsupported,
    _Unauthorizedforcollateralinquiry,
    _Other,
}
impl Field for StrikeCurrency {
    fn tag(&self) -> u16 {
        return 947;
    }
}
pub struct StrikeCurrency {
    value: f32,
}
impl Field for NoNested3PartyIDs {
    fn tag(&self) -> u16 {
        return 948;
    }
}
pub struct NoNested3PartyIDs {
    value: u16,
}
impl Field for Nested3PartyID {
    fn tag(&self) -> u16 {
        return 949;
    }
}
pub struct Nested3PartyID {
    value: String,
}
impl Field for Nested3PartyIDSource {
    fn tag(&self) -> u16 {
        return 950;
    }
}
pub struct Nested3PartyIDSource {
    value: char,
}
impl Field for Nested3PartyRole {
    fn tag(&self) -> u16 {
        return 951;
    }
}
pub struct Nested3PartyRole {
    value: u16,
}
impl Field for NoNested3PartySubIDs {
    fn tag(&self) -> u16 {
        return 952;
    }
}
pub struct NoNested3PartySubIDs {
    value: u16,
}
impl Field for Nested3PartySubID {
    fn tag(&self) -> u16 {
        return 953;
    }
}
pub struct Nested3PartySubID {
    value: String,
}
impl Field for Nested3PartySubIDType {
    fn tag(&self) -> u16 {
        return 954;
    }
}
pub struct Nested3PartySubIDType {
    value: u16,
}
impl Field for LegContractSettlMonth {
    fn tag(&self) -> u16 {
        return 955;
    }
}
pub struct LegContractSettlMonth {
    value: u8,
}
impl Field for LegInterestAccrualDate {
    fn tag(&self) -> u16 {
        return 956;
    }
}
pub struct LegInterestAccrualDate {
    value: u64,
}
