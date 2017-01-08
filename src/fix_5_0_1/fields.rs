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
    _Percent,
    _Absolute,
    _Percentagewaived4,
    _Percentagewaived5,
    _Pointsperbondorcontract,
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
    _Fixedpegtolocalbestbidorofferattimeoforder,
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
    _Intermarketsweep,
    _Externalroutingallowed,
    _Externalroutingnotallowed,
    _Imbalanceonly,
    _Singleexecutionrequestedforblocktrade,
    _Bestexecution,
    _Suspendonsystemfailure,
    _Suspendontradinghalt,
    _Reinstateonconnectionloss,
    _Cancelonconnectionloss,
    _Suspendonconnectionloss,
    _Releasefromsuspension,
    _Executeasdeltaneutralusingvolatilityprovided,
    _Executeasdurationneutral,
    _Executeasfxneutral,
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
    _Optionpricereportingauthority,
    _Isdafpmlproducturl,
    _Letterofcredit,
    _Marketplaceassignedidentifier,
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
    _Undisclosedquantity,
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
    _Ioi,
    _Advertisement,
    _Executionreport,
    _Ordercancelreject,
    _Logon,
    _Derivativesecuritylist,
    _Newordermultileg,
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
    _News,
    _Collateralreport,
    _Collateralinquiry,
    _Networkcounterpartysystemstatusrequest,
    _Networkcounterpartysystemstatusresponse,
    _Userrequest,
    _Userresponse,
    _Collateralinquiryack,
    _Confirmationrequest,
    _Tradingsessionlistrequest,
    _Tradingsessionlist,
    _Securitylistupdatereport,
    _Adjustedpositionreport,
    _Allocationinstructionalert,
    _Executionacknowledgement,
    _Contraryintentionreport,
    _Securitydefinitionupdatereport,
    _Settlementobligationreport,
    _Derivativesecuritylistupdatereport,
    _Tradingsessionlistupdatereport,
    _Marketdefinitionrequest,
    _Marketdefinition,
    _Marketdefinitionupdatereport,
    _Applicationmessagerequest,
    _Applicationmessagerequestack,
    _Applicationmessagereport,
    _Ordermassactionreport,
    _Email,
    _Ordermassactionrequest,
    _Usernotification,
    _Newordersingle,
    _Neworderlist,
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
    _Xmlnonfix,
    _Registrationinstructions,
    _Registrationinstructionsresponse,
    _Ordermasscancelrequest,
    _Ordermasscancelreport,
    _Newordercross,
    _Crossordercancelreplacerequest,
    _Crossordercancelrequest,
    _Securitytyperequest,
    _Securitytypes,
    _Securitylistrequest,
    _Securitylist,
    _Derivativesecuritylistrequest,
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
    _Forexmarket,
    _Previouslyquoted,
    _Previouslyindicated,
    _Forexlimit,
    _Forexswap,
    _Forexpreviouslyquoted,
    _Funari,
    _Marketiftouched,
    _Marketwithleftoveraslimit,
    _Previousfundvaluationpoint,
    _Nextfundvaluationpoint,
    _Pegged,
    _Counterorderselection,
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
    _Crossshortexxmpt,
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
    _Goodthroughcrossing,
    _Atcrossing,
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
    _Brokendate,
    _Fxspotnextsettlement,
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
#[derive(Debug)]
pub enum SymbolSfx {
    _Eucpwithlumpsuminterestratherthandiscountprice,
    _Whenissuedforasecuritytobereissuedunderanoldcusiporisin,
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
    _Preliminary,
    _Calculated,
    _Calculatedwithoutpreliminary,
    _Reversal,
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
    _Close,
    _Fifo,
    _Open,
    _Rolled,
    _Closebutnotifyonopen,
    _Default,
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
    _Allocationpending,
    _Reversed,
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
    _Incorrectaveragegprice,
    _Unknownexecutingbrokermnemonic,
    _Commissiondifference,
    _Unknownorderid,
    _Unknownlistid,
    _Other,
    _Incorrectallocatedquantity,
    _Calculationdifference,
    _Unknownorstaleexecid,
    _Mismatcheddata,
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
    _None,
    _Pkcs1,
    _Des,
    _Pkcs3,
    _Pgp4,
    _Pgp5,
    _Pem,
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
    _Priceexceedscurrentprice,
    _Priceexceedscurrentpriceband,
    _Invalidpriceincrement,
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
    _Unsupportedordercharacteristic,
    _Surveillenceoption,
    _Incorrectquantity,
    _Incorrectallocatedquantity,
    _Unknownaccount,
    _Priceexceedscurrentpriceband,
    _Invalidpriceincrement,
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
    _Indidcation,
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
    _No,
    _Yes,
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
    _Transferfee,
    _Securitylending,
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
    _No,
    _Yes,
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
    _Replaced,
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
    _Tradeinaclearinghold,
    _Tradehasbeenreleasedtoclearing,
    _Triggeredoractivatedbysystem,
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
    _Default,
    _Standinginstructionsprovided,
    _Specificallocationaccountoverriding,
    _Specificallocationaccountstanding,
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
    _Ustreasurynoteust,
    _Ustreasurybillustb,
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
    _Eurocorporatefloatingratenotes,
    _Uscorporatefloatingratenotes,
    _Indexedlinked,
    _Structurednotes,
    _Yankeecorporatebond,
    _Foreignexchangecontract,
    _Creditdefaultswap,
    _Future,
    _Option,
    _Optionsonfutures,
    _Optionsonphysical,
    _Interestrateswap,
    _Optionsoncombo,
    _Commonstock,
    _Preferredstock,
    _Repurchase,
    _Forward,
    _Buysellback,
    _Securitiesloan,
    _Securitiespledge,
    _Bradybond,
    _Canadiantreasurynotes,
    _Canadiantreasurybills,
    _Eurosovereigns,
    _Canadianprovincialbonds,
    _Treasurybill,
    _Ustreasurybond,
    _Intereststripfromanybondornote,
    _Ustreasurybilltbill,
    _Treasuryinflationprotectedsecurities,
    _Principalstripofacallablebondornote,
    _Principalstripfromanoncallablebondornote,
    _Ustreasurynotetnote,
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
    _Bankdepositorynote,
    _Banknotes,
    _Billofexchanges,
    _Canadianmoneymarkets,
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
    _Shorttermloannote,
    _Plazosfijos,
    _Securedliquiditynote,
    _Timedeposit,
    _Termliquiditynote,
    _Extendedcommnote,
    _Yankeecertificateofdeposit,
    _Assetbackedsecurities,
    _Canadianmortgagebonds,
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
    _Otheranticipationnotes,
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
    _Taxablemunicipalcp,
    _Taxrevenueanticipationnote,
    _Variableratedemandnote,
    _Warrant,
    _Mutualfund,
    _Multileginstrument,
    _Nosecuritytype,
    _Wildcardentryforuseonsecuritydefinitionrequest,
    _Cash,
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
#[derive(Debug)]
pub enum BenchmarkCurveName {
    _Eonia,
    _Eurepo,
    _Euribor,
    _Futureswap,
    _Libid,
    _Libor,
    _Muniaaa,
    _Other,
    _Pfandbriefe,
    _Sonia,
    _Swap,
    _Treasury,
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
    _Alternativeminimumtax,
    _Autoreinvestmentatrateorbetter,
    _Bankqualified,
    _Bargainconditions,
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
    _Minimumdenomination,
    _Minimumincrement,
    _Minimumquantity,
    _Paymentfrequencycalendar,
    _Numberofpieces,
    _Poolsmaximum,
    _Poolsperlot,
    _Poolspermillion,
    _Poolspertrade,
    _Pricerange,
    _Pricingfrequency,
    _Productionyear,
    _Callprotection,
    _Purpose,
    _Benchmarkpricesource,
    _Ratingsourceandrange,
    _Typeofredemption,
    _Restricted,
    _Marketsector,
    _Securitytypeincludedorexcluded,
    _Structure,
    _Substitutionsfrequency,
    _Substitutionsleft,
    _Freeformtext,
    _Tradevariance,
    _Weightedaveragecoupon,
    _Weightedaveragelifecoupon,
    _Weightedaverageloanage,
    _Weightedaveragematurity,
    _Wholepool,
    _Yieldrange,
    _Averageficoscore,
    _Averageloansize,
    _Maximumloanbalance,
    _Poolidentifier,
    _Typeofrolltrade,
    _Referencetorollingorclosingtrade,
    _Principalofrollingorclosingtrade,
    _Interestofrollingorclosingtrade,
    _Availableofferquantitytobeshowntothestreet,
    _Brokerssalescredit,
    _Offerpricetobeshowntointernalbrokers,
    _Offerquantitytobeshowntointernalbrokers,
    _Theminimumresidualofferquantity,
    _Maximumordersize,
    _Orderquantityincrement,
    _Primaryorsecondarymarketindicator,
    _Brokersalescreditoverride,
    _Traderscredit,
    _Discountrate,
    _Yieldtomaturity,
    _Absoluteprepaymentspeed,
    _Constantprepaymentpenalty,
    _Constantprepaymentrate,
    _Constantprepaymentyield,
    _Finalcprofhomeequityprepaymentcurve,
    _Percentofmanufacturedhousingprepaymentcurve,
    _Monthlyprepaymentrate,
    _Percentofprospectusprepaymentcurve,
    _Percentofbmaprepaymentcurve,
    _Singlemonthlymortality,
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
    _Yieldtoavgmaturity,
    _Bookyield,
    _Yieldtonextcall,
    _Yieldchangesinceclose,
    _Closingyield,
    _Compoundyield,
    _Currentyield,
    _Gvntequivalentyield,
    _Truegrossyield,
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
    _Previouscloseyield,
    _Proceedsyield,
    _Yieldtonextput,
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
    _No,
    _Yes,
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
    _Compositeunderlyingprice,
    _Simulatedsellprice,
    _Simulatedbuyprice,
    _Marginrate,
    _Midprice,
    _Emptybook,
    _Settlehighprice,
    _Settlelowprice,
    _Priorsettleprice,
    _Sessionhighbid,
    _Sessionlowoffer,
    _Earlyprices,
    _Auctionclearingprice,
    _Swapvaluefactor,
    _Dailyvalueadjustmentforlongpositions,
    _Cumulativevalueadjustmentforlongpositions,
    _Dailyvalueadjustmentforshortpositions,
    _Cumulativevalueadjustmentforshortpositions,
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
    _Openactive,
    _Closedinactive,
    _Exchangebest,
    _Consolidatedbest,
    _Locked,
    _Crossed,
    _Depth,
    _Fasttrading,
    _Nonfirm,
    _Manualslowquote,
    _Outrightprice,
    _Impliedprice,
    _Depthonoffer,
    _Depthonbid,
    _Closing,
    _Newsdissemination,
    _Tradingrange,
    _Orderinflux,
    _Duetorelated,
    _Newspending,
    _Additionalinfo,
    _Additionalinfoduetorelated,
    _Resume,
    _Viewofcommon,
    _Volumealert,
    _Orderimbalance,
    _Equipmentchangeover,
    _Noopen,
    _Regulareth,
    _Automaticexecution,
    _Automaticexecutioneth,
    _Fastmarketeth,
    _Inactiveeth,
    _Rotation,
    _Rotationeth,
    _Halt,
    _Halteth,
    _Duetonewsdissemination,
    _Duetonewspending,
    _Tradingresume,
    _Outofsequence,
    _Bidspecialist,
    _Offerspecialist,
    _Bidofferspecialist,
    _Endofdaysam,
    _Forbiddensam,
    _Frozensam,
    _Preopeningsam,
    _Openingsam,
    _Opensam,
    _Surveillancesam,
    _Suspendedsam,
    _Reservedsam,
    _Noactivesam,
    _Restricted,
    _Restofbookvwap,
    _Betterpricesinconditionalorders,
    _Medianprice,
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
    _Openingreopeningtradedetail,
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
    _Bargaincondition,
    _Convertedpriceindicator,
    _Exchangelast,
    _Finalpriceofsession,
    _Expit,
    _Crossedx,
    _Tradesresultingfrommanualslowquote,
    _Tradesresultingfromintermarketsweep,
    _Volumeonly,
    _Directplus,
    _Acquisition,
    _Bunched,
    _Distribution,
    _Bunchedsale,
    _Splittrade,
    _Cancelstopped,
    _Canceleth,
    _Cancelstoppedeth,
    _Outofsequenceeth,
    _Cancellasteth,
    _Soldlastsaleeth,
    _Cancellast,
    _Soldlastsale,
    _Cancelopen,
    _Cancelopeneth,
    _Openedsaleeth,
    _Cancelonly,
    _Cancelonlyeth,
    _Lateopeneth,
    _Autoexecutioneth,
    _Reopen,
    _Reopeneth,
    _Adjusted,
    _Adjustedeth,
    _Spread,
    _Spreadeth,
    _Straddle,
    _Straddleeth,
    _Stopped,
    _Stoppedeth,
    _Regulareth,
    _Combo,
    _Comboeth,
    _Officialclosingprice,
    _Priorreferenceprice,
    _Cancel,
    _Stoppedsoldlast,
    _Stoppedoutofsequence,
    _Officalclosingprice,
    _Crossedao,
    _Fastmarket,
    _Automaticexecution,
    _Formt,
    _Basketindex,
    _Burstbasket,
    _Outsidespread,
    _Impliedtrade,
    _Marketplaceenteredtrade,
    _Multassetclassmultilegtrade,
    _Multilegtomultilegtrade,
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
    _Deletethru,
    _Deletefrom,
    _Overlay,
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
    _Insufficientcredit,
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
    _Cancellation,
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
    _Restricted,
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
    _Cashdividend,
    _Stockdividend,
    _Nonintegerstocksplit,
    _Reversestocksplit,
    _Standardintegerstocksplit,
    _Positionconsolidation,
    _Liquidationreorganization,
    _Mergerreorganization,
    _Rightsoffering,
    _Shareholdermeeting,
    _Spinoff,
    _Tenderoffer,
    _Warrant,
    _Specialaction,
    _Symbolconversion,
    _Cusip,
    _Leaprollover,
    _Successionevent,
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
    _Cancelforsymbol,
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
    _Active,
    _Canceled,
    _Unsolicitedquotereplenishment,
    _Pendingendtrade,
    _Toolatetoend,
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
    _Cancelquotespecifiedinquoteid,
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
    _Priceexceedscurrentpriceband,
    _Quotelocked,
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
    _Summaryacknowledgement,
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
    _Symbol,
    _Securitytypeandorcficode,
    _Product,
    _Tradingsessionid,
    _Allsecurities,
    _Marketidormarketidplusmarketsegmentid,
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
    _Listofsecuritytypesreturnedperrequest,
    _Listofsecuritiesreturnedperrequest,
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
    _No,
    _Yes,
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
    _Noopen,
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
    _Precross,
    _Cross,
}
impl Field for HaltReasonChar {
    fn tag(&self) -> u16 {
        return 327;
    }
}
#[derive(Debug)]
pub enum HaltReasonChar {
    _Newsdissemination,
    _Orderinflux,
    _Orderimbalance,
    _Additionalinformation,
    _Newpending,
    _Equipmentchangeover,
}
impl Field for InViewOfCommon {
    fn tag(&self) -> u16 {
        return 328;
    }
}
#[derive(Debug)]
pub enum InViewOfCommon {
    _No,
    _Yes,
}
impl Field for DueToRelated {
    fn tag(&self) -> u16 {
        return 329;
    }
}
#[derive(Debug)]
pub enum DueToRelated {
    _No,
    _Yes,
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
#[derive(Debug)]
pub enum TradingSessionID {
    _Day,
    _Halfday,
    _Morning,
    _Afternoon,
    _Evening,
    _Afterhours,
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
pub struct MessageEncoding {
    value: String,
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
    _Invalidunsupportedapplicationversion,
    _Other,
}
impl Field for BidRequestTransType {
    fn tag(&self) -> u16 {
        return 374;
    }
}
#[derive(Debug)]
pub enum BidRequestTransType {
    _Cancel,
    _No,
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
    _Pegrefresh,
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
    _Unknownid,
    _Unknownsecurity,
    _Unknownmessagetype,
    _Applicationnotavailable,
    _Conditionallyrequiredfieldmissing,
    _Notauthorized,
    _Delivertofirmnotavailableatthistime,
    _Invalidpriceincrement,
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
    _Receive,
    _Send,
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
    _Averagepriceguarantee,
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
    _Disclosedsytle,
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
    _Buysideexplicitlyrequestsstatususingstatuerequest,
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
    _Agency,
    _Vwapguarantee,
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
    _Discount,
    _Premium,
    _Spread,
    _Tedprice,
    _Tedyield,
    _Yield,
    _Fixedcabinettradeprice,
    _Variablecabinettradeprice,
    _Productticksinhalfs,
    _Productticksinfourths,
    _Productticksineights,
    _Productticksinsixteenths,
    _Productticksinthirtyseconds,
    _Productticksinsixtyforths,
    _Productticksinonetwentyeights,
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
    _Accumulateexectuionsuntilforderisfilledorexpires,
    _Accumulateuntilverballlynotifiedotherwise,
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
    _Cancelling,
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
    _Waitforexecutinstruction,
    _Exchangeswitchcivorder3,
    _Exchangeswitchcivorder4,
    _Exchangeswitchcivorder5,
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
    _Uknationalinsuranceorpensionnumber,
    _Ussocialsecuritynumber,
    _Usemployerortaxidnumber,
    _Australianbusinessnumber,
    _Australiantaxfilenumber,
    _Koreaninvestorid,
    _Taiwanesequalifiedforeigninvestoridqfiifid,
    _Taiwanesetradingacct,
    _Malaysiancentraldepository,
    _Chineseinvestorid,
    _Directedbrokerthreecharacteracronymasdefinedinisitcetcbestpracticeguidelinesdocument,
    _Bic,
    _Generallyacceptedmarketparticipantidentifier,
    _Proprietary,
    _Isocountrycode,
    _Settlemententitylocation,
    _Mic,
    _Csdparticipantmembercode,
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
    _Locate,
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
    _Contrainvestorid,
    _Transfertofirm,
    _Contrapositionaccount,
    _Contraexchange,
    _Internalcarryaccount,
    _Orderentryoperatorid,
    _Secondaryaccountnumber,
    _Foriegnfirm,
    _Thirdpartyallocationfirm,
    _Claimingaccount,
    _Assetmanager,
    _Pledgoraccount,
    _Pledgeeaccount,
    _Largetraderreportableaccount,
    _Tradermnemonic,
    _Senderlocation,
    _Sessionid,
    _Acceptablecounterparty,
    _Unacceptablecounterparty,
    _Enteringunit,
    _Executingunit,
    _Introducingbroker,
    _Quoteoriginator,
    _Reportoriginator,
    _Systematicinternaliser,
    _Multilateraltradingfacility,
    _Regulatedmarket,
    _Marketmaker,
    _Investmentfirm,
    _Hostcompetentauthority,
    _Homecompetentauthority,
    _Competentauthorityofthemostrelevantmarketintermsofliquidity,
    _Competentauthorityofthetransaction,
    _Reportingintermediary,
    _Executionvenue,
    _Marketdataentryoriginator,
    _Locationid,
    _Deskid,
    _Marketdatamarket,
    _Allocationentity,
    _Primebrokerprovidinggeneraltradeservices,
    _Stepoutfirm,
    _Brokerclearingid,
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
    _No,
    _Yes,
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
    _Highvalueclearingsystemhvacs,
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
    _Non,
    _Nom,
    _Noo,
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
    _Exempt1,
    _Exempt2,
    _Exempt3,
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
    _Creationpriceplusadjustmentpercent,
    _Creationpriceplusadjustmentamount,
    _Offerprice,
    _Offerpriceminusadjustmentpercent,
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
#[derive(Debug)]
pub enum TradeReportTransType {
    _New,
    _Cancel,
    _Replace,
    _Release,
    _Reverse,
    _Cancelduetobackoutoftrade,
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
    _Employee9,
    _Employee10,
    _Employer11,
    _Employer12,
    _Nonfundprototypeira,
    _Nonfundqualifiedplan,
    _Definedcontributionplan,
    _Individualretirementaccount16,
    _Individualretirementaccount17,
    _Keogh,
    _Profitsharingplan,
    _401,
    _Selfdirectedira,
    _403,
    _457,
    _Rothira24,
    _Rothira25,
    _Rothconversionira26,
    _Rothconversionira27,
    _Educationira28,
    _Educationira29,
    _Other,
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
    _Reminder,
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
    _Invalidunacceptablenoregdetails,
    _Invalidunacceptableregseqno,
    _Invalidunacceptableregdetails,
    _Invalidunacceptablemailingdetails,
    _Invalidunacceptablemailinginstructions,
    _Invalidunacceptableinvestorid,
    _Invalidunaceeptableinvestoridsource,
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
    _Cancel,
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
    _Commissionpercent,
    _Initialchargeamount,
    _Initialchargepercent,
    _Discountamount,
    _Discountpercent,
    _Dilutionlevyamount,
    _Dilutionlevypercent,
    _Exitchargeamount,
    _Exitchargepercent,
    _Fundbasedrenewalcommissionpercent,
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
    _Actingasmarketmakerofspecialistintheunderlyingsecurityofaderivativeseucirty,
    _Foreignentity,
    _Externalmarketparticipant,
    _Extneralinterconnectedmarketlinkage,
    _Risklessarbitrage,
    _Issuerholding,
    _Issuepricestabilization,
    _Nonalgorithmic,
    _Algorithmic,
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
    _Cancelordersforamarket,
    _Cancelordersforamarketsegment,
    _Cancelordersforasecuritygroup,
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
    _Cancelordersforamarket,
    _Cancelordersforamarketsegment,
    _Cancelordersforasecuritygroup,
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
    _Invalidorunkownunderlyingsecurity,
    _Invalidorunknownproduct,
    _Invalidorunknowncficode,
    _Invalidorunknownsecuritytype,
    _Invalidorunknowntradingsession,
    _Invalidorunknownmarket,
    _Invalidorunkownmarketsegment,
    _Invalidorunknownsecuritygroup,
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
    _Localmarket,
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
    _No,
    _Yes,
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
pub enum CrossType {
    _Crossaon,
    _Crossioc,
    _Crossoneside,
    _Crosssameprice,
}
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
    _Marketidormarketidplusmarketsegmentid,
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
    _Uncomparedunmatchedorunaffired,
    _Advisoryoralert,
}
impl Field for MatchType {
    fn tag(&self) -> u16 {
        return 574;
    }
}
#[derive(Debug)]
pub enum MatchType { _Onepartytradereport,_Twopartytradereport,_Confirmedtradereport,_Automatch,_Crossauction,_Counterorderselection,_Callauction,_Issuingbuybackauction,_Actacceptedtrade,_Actdefaulttrade,_Actdefaultafterm2,_Actm6Match,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusfourbadgesandexecutiontime,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusfourbadges,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplustwobadgesandexecutiontime,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplustwobadges,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorplusexecutiontime,_Comparedrecordsresultingfromstampedadvisoriesorspecialistacceptspairoffs,_Summarizedmatchusinga1Exactmatchcriteriaexceptquantityissummaried,_Summarizedmatchusinga2Exactmatchcriteriaexceptquantityissummarized,_Summarizedmatchusinga3Exactmatchcriteriaexceptquantityissummarized,_Summarizedmatchusinga4Exactmatchcriteriaexceptquantityissummarized,_Summarizedmatchusinga5Exactmatchcriteriaexceptquantityissummarized,_Exactmatchontradedatestocksymbolquantitypricetradetypeandspecialtradeindicatorminusbadgesandtimesactm1Match,_Summarizedmatchminusbadgesandtimesactm2Match,_Ocslockedinnonact }
impl Field for OddLot {
    fn tag(&self) -> u16 {
        return 575;
    }
}
#[derive(Debug)]
pub enum OddLot {
    _No,
    _Yes,
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
    _Qualifiedservicerepresentativeqsr,
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
    _Accountiscarriedoncustomersideofthebooks,
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
    _Donotprorata,
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
#[derive(Debug)]
pub enum TradingSessionSubID {
    _Pretrading,
    _Openingoropeningauction,
    _3,
    _Closingorclosingauction,
    _Posttrading,
    _Intradayauction,
    _Quiescent,
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
    _Sellsidecalculatedusingpreliminary,
    _Sellsidecalculatedwithoutpreliminary,
    _Readytobook,
    _Buysidereadytobook,
    _Warehouseinstruction,
    _Requesttointermediary,
    _Accept,
    _Reject,
    _Acceptpending,
    _Incompletegroup,
    _Completegroup,
    _Reversalpending,
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
    _1Styeardelegatetradingforownaccount,
    _2Ndyeardelegatetradingforownaccount,
    _3Rdyeardelegatetradingforownaccount,
    _4Thyeardelegatetradingforownaccount,
    _5Thyeardelegatetradingforownaccount,
    _6Thyeardelegatetradingforownaccount,
    _Cboemember,
    _Nonmemberandcustomer,
    _Equitymemberandclearingmember,
    _Fullandassociatemembertradingforownaccountandasfloorbrokers,
    _106Hand106Jfirms,
    _Gimidemandcommembershipinterestholders,
    _Lessee106Femployees,
    _Allotherownershiptypes,
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
    _No,
    _Yes,
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
    _Insufficientcredit,
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
impl Field for LegOrderQty {
    fn tag(&self) -> u16 {
        return 685;
    }
}
pub struct LegOrderQty {
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
    _Discount,
    _Premium,
    _Spread,
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
    _Endtrade,
    _Timedout,
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
    _Allocationtradeqty,
    _Optionassignment,
    _Asoftradeqty,
    _Deliveryqty,
    _Electronictradeqty,
    _Optionexerciseqty,
    _Endofdayqty,
    _Intraspreadqty,
    _Interspreadqty,
    _Adjustmentqty,
    _Pittradeqty,
    _Startofdayqty,
    _Integralsplit,
    _Transactionfromassignment,
    _Totaltransactionqty,
    _Transactionquantity,
    _Transfertradeqty,
    _Transactionfromexercise,
    _Crossmarginqty,
    _Receivequantity,
    _Corporateactionadjustment,
    _Deliverynoticeqty,
    _Exchangeforphysicalqty,
    _Privatelynegotiatedtradeqty,
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
    _Cashamount,
    _Cashresidualamount,
    _Finalmarktomarketamount,
    _Incrementalmarktomarketamount,
    _Premiumamount,
    _Startofdaymarktomarketamount,
    _Tradevariationamount,
    _Valueadjustedamount,
    _Settlementvalue,
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
    _Largetradersubmission,
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
    _New,
    _Replace,
    _Cancel,
    _Reverse,
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
    _Endofday,
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
    _Settlementactivity,
    _Backoutmessage,
}
impl Field for ResponseTransportType {
    fn tag(&self) -> u16 {
        return 725;
    }
}
#[derive(Debug)]
pub enum ResponseTransportType {
    _Inband,
    _Outofband,
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
    _Prorata,
    _Random,
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
    _Notauthorized,
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
    _Invalidpartyonformation,
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
    _Deskreceipt,
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
    _Cash,
    _Securities,
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
    _Preliminaryrequesttointermediary,
    _Sellsidecalculatedusingpreliminary,
    _Sellsidecalculatedwithoutpreliminary,
    _Warehouserecap,
    _Requesttointermediary,
    _Accept,
    _Reject,
    _Acceptpending,
    _Complete,
    _Reversepending,
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
    _Accountiscarriedpncustomersideofbooks,
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
    _Locationdesk,
    _Positionaccounttype,
    _Securitylocateid,
    _Marketmaker,
    _Eligiblecounterparty,
    _Professionalclient,
    _Location,
    _Executionvenue,
    _Currencydeliveryidentifier,
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
    _Lasttradeistheaveragepricegroupidentifiedbythetradelinkid,
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
    _Allocationgiveupexecutor,
    _Allocationfromexecutor,
    _Allocationtoclaimaccount,
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
    _Tradingeligibilityexpirationspecifiedinthedateandtimefieldseventdate,
}
impl Field for TrdType {
    fn tag(&self) -> u16 {
        return 828;
    }
}
#[derive(Debug)]
pub enum TrdType {
    _Regulartrade,
    _Blocktrade1,
    _Efp,
    _Transfer,
    _Latetrade,
    _Ttrade,
    _Weightedaveragepricetrade,
    _Bunchedtrade,
    _Latebunchedtrade,
    _Priorreferencepricetrade,
    _Afterhourstrade,
    _Exchangeforrisk,
    _Exchangeforswap,
    _Exchangeoffuturesfor,
    _Exchangeofoptionsforoptions,
    _Tradingatsettlement,
    _Allornone,
    _Futureslargeorderexecution,
    _Exchangeoffuturesforfutures,
    _Optioninterimtrade,
    _Optioncabinettrade,
    _Privatelynegotiatedtrades,
    _Substitutionoffuturesforforwards,
    _Nonstandardsettlement,
    _Derivativerelatedtransaction,
    _Portfoliotrade,
    _Volumeweightedaveragetrade,
    _Exchangegrantedtrade,
    _Repurchaseagreement,
    _Otc,
    _Exchangebasisfacility,
    _Errortrade,
    _Specialcumdividend,
    _Specialexdividend,
    _Specialcumcoupon,
    _Specialexcoupon,
    _Cashsettlement,
    _Specialprice,
    _Guaranteeddelivery,
    _Specialcumrights,
    _Specialexrights,
    _Specialcumcapitalrepayments,
    _Specialexcapitalrepayments,
    _Specialcumbonus,
    _Specialexbonus,
    _Blocktrade38,
    _Workedprincipaltrade,
    _Blocktrades,
    _Namechange,
    _Portfoliotransfer,
    _Prorogationbuy,
    _Prorogationsell,
    _Optionexercise,
    _Deltaneutraltransaction,
    _Financingtransaction,
}
impl Field for TrdSubType {
    fn tag(&self) -> u16 {
        return 829;
    }
}
#[derive(Debug)]
pub enum TrdSubType {
    _Cmta,
    _Internaltransferoradjustment,
    _Externaltransferortransferofaccount,
    _Rejectforsubmittingside,
    _Advisoryforcontraside,
    _Offsetduetoanallocation,
    _Onsetduttoanallocation,
    _Differentialspread,
    _Impliedspreadlegexecutedagainstanoutright,
    _Transactionfromexercise,
    _Transactionfromassignment,
    _Acats,
    _Offhourstrade,
    _Onhourstrade,
    _Otcquote,
    _Convertedswap,
    _Ai,
    _B,
    _K,
    _Lc,
    _M,
    _N,
    _Nm,
    _Nr,
    _P,
    _Pa,
    _Pc,
    _Pn,
    _R,
    _Ro,
    _Rt,
    _Sw,
    _T,
    _Wn,
    _Wt,
    _Crossedtrade,
    _Interimprotectedtrade,
    _Largeinscale,
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
    _Strict,
    _Orworse,
}
impl Field for PegRoundDirection {
    fn tag(&self) -> u16 {
        return 838;
    }
}
#[derive(Debug)]
pub enum PegRoundDirection {
    _Moreaggressive,
    _Morepassive,
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
    _Strict,
    _Orworse,
}
impl Field for DiscretionRoundDirection {
    fn tag(&self) -> u16 {
        return 844;
    }
}
#[derive(Debug)]
pub enum DiscretionRoundDirection {
    _Moreaggressive,
    _Morepassive,
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
    _Auction,
}
impl Field for PublishTrdIndicator {
    fn tag(&self) -> u16 {
        return 852;
    }
}
#[derive(Debug)]
pub enum PublishTrdIndicator {
    _No,
    _Yes,
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
    _Qualifiedservicerepresentative,
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
    _Unitsofmeasurepertimeunit,
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
    _Alleged1,
    _Accept,
    _Decline,
    _Addendum,
    _Nowas,
    _Tradereportcancel,
    _7,
    _Defaulted,
    _Invalidcmta,
    _Pended,
    _Allegednew,
    _Allegedaddendum,
    _Allegednowas,
    _Allegedtradereportcancel,
    _Alleged15,
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
    _Activation,
    _Inactiviation,
    _Lasteligibletradedate,
    _Swapstartdate,
    _Swapenddate,
    _Swaprolldate,
    _Swapnextstartdate,
    _Swapnextrolldate,
    _Firstdeliverydate,
    _Lastdeliverydate,
    _Initialinventoryduedate,
    _Finalinventoryduedate,
    _Firstintentdate,
    _Lastintentdate,
    _Positionremovaldate,
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
    _Escrowedtoredemptiondate,
    _Prerefunded,
    _Indefault,
    _Unrated,
    _Taxable,
    _Indexed,
    _Subjecttoalternativeminimumtax,
    _Originalissuediscountpricesupplypriceintheinstrattribvalue,
    _Callablebelowmaturityvalue,
    _Callablewithoutnoticebymailtoholderunlessregistered,
    _Pricetickrulesforsecurity,
    _Tradetypeeligibilitydetailsforsecurity,
    _Instrumentdenominator,
    _Instrumentnumerator,
    _Instrumentpriceprecision,
    _Instrumentstrikeprice,
    _Tradeableindicator,
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
    _No,
    _Yes,
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
#[derive(Debug)]
pub enum LastRptRequested {
    _No,
    _Yes,
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
    _Forceduserlogoutbyexchange,
    _Sessionshutdownwarning,
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
    _Notconnected2,
    _Notconnected3,
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
    _Acceptedwitherrors,
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
impl Field for NoStrategyParameters {
    fn tag(&self) -> u16 {
        return 957;
    }
}
pub struct NoStrategyParameters {
    value: u16,
}
impl Field for StrategyParameterName {
    fn tag(&self) -> u16 {
        return 958;
    }
}
pub struct StrategyParameterName {
    value: String,
}
impl Field for StrategyParameterType {
    fn tag(&self) -> u16 {
        return 959;
    }
}
#[derive(Debug)]
pub enum StrategyParameterType {
    _Int,
    _Length,
    _Numingroup,
    _Seqnum,
    _Tagnum,
    _Float,
    _Qty,
    _Price,
    _Priceoffset,
    _Amt,
    _Percentage,
    _Char,
    _Boolean,
    _String,
    _Multiplecharvalue,
    _Currency,
    _Exchange,
    _Monthyear,
    _Utctimestamp,
    _Utctimeonly,
    _Localmkttime,
    _Utcdate,
    _Data,
    _Multiplestringvalue,
}
impl Field for StrategyParameterValue {
    fn tag(&self) -> u16 {
        return 960;
    }
}
pub struct StrategyParameterValue {
    value: String,
}
impl Field for HostCrossID {
    fn tag(&self) -> u16 {
        return 961;
    }
}
pub struct HostCrossID {
    value: String,
}
impl Field for SideTimeInForce {
    fn tag(&self) -> u16 {
        return 962;
    }
}
pub struct SideTimeInForce {
    value: u64,
}
impl Field for MDReportID {
    fn tag(&self) -> u16 {
        return 963;
    }
}
pub struct MDReportID {
    value: u16,
}
impl Field for SecurityReportID {
    fn tag(&self) -> u16 {
        return 964;
    }
}
pub struct SecurityReportID {
    value: u16,
}
impl Field for SecurityStatus {
    fn tag(&self) -> u16 {
        return 965;
    }
}
#[derive(Debug)]
pub enum SecurityStatus {
    _Active,
    _Inactive,
}
impl Field for SettleOnOpenFlag {
    fn tag(&self) -> u16 {
        return 966;
    }
}
pub struct SettleOnOpenFlag {
    value: String,
}
impl Field for StrikeMultiplier {
    fn tag(&self) -> u16 {
        return 967;
    }
}
pub struct StrikeMultiplier {
    value: f32,
}
impl Field for StrikeValue {
    fn tag(&self) -> u16 {
        return 968;
    }
}
pub struct StrikeValue {
    value: f32,
}
impl Field for MinPriceIncrement {
    fn tag(&self) -> u16 {
        return 969;
    }
}
pub struct MinPriceIncrement {
    value: f32,
}
impl Field for PositionLimit {
    fn tag(&self) -> u16 {
        return 970;
    }
}
pub struct PositionLimit {
    value: u16,
}
impl Field for NTPositionLimit {
    fn tag(&self) -> u16 {
        return 971;
    }
}
pub struct NTPositionLimit {
    value: u16,
}
impl Field for UnderlyingAllocationPercent {
    fn tag(&self) -> u16 {
        return 972;
    }
}
pub struct UnderlyingAllocationPercent {
    value: f32,
}
impl Field for UnderlyingCashAmount {
    fn tag(&self) -> u16 {
        return 973;
    }
}
pub struct UnderlyingCashAmount {
    value: f32,
}
impl Field for UnderlyingCashType {
    fn tag(&self) -> u16 {
        return 974;
    }
}
#[derive(Debug)]
pub enum UnderlyingCashType {
    _Fixed,
    _Diff,
}
impl Field for UnderlyingSettlementType {
    fn tag(&self) -> u16 {
        return 975;
    }
}
#[derive(Debug)]
pub enum UnderlyingSettlementType {
    _Tplus1,
    _Tplus3,
    _Tplus4,
}
impl Field for QuantityDate {
    fn tag(&self) -> u16 {
        return 976;
    }
}
pub struct QuantityDate {
    value: u64,
}
impl Field for ContIntRptID {
    fn tag(&self) -> u16 {
        return 977;
    }
}
pub struct ContIntRptID {
    value: String,
}
impl Field for LateIndicator {
    fn tag(&self) -> u16 {
        return 978;
    }
}
pub struct LateIndicator {
    value: bool,
}
impl Field for InputSource {
    fn tag(&self) -> u16 {
        return 979;
    }
}
pub struct InputSource {
    value: String,
}
impl Field for SecurityUpdateAction {
    fn tag(&self) -> u16 {
        return 980;
    }
}
#[derive(Debug)]
pub enum SecurityUpdateAction {
    _Add,
    _Delete,
    _Modify,
}
impl Field for NoExpiration {
    fn tag(&self) -> u16 {
        return 981;
    }
}
pub struct NoExpiration {
    value: u16,
}
impl Field for ExpirationQtyType {
    fn tag(&self) -> u16 {
        return 982;
    }
}
#[derive(Debug)]
pub enum ExpirationQtyType {
    _Autoexercise,
    _Nonautoexercise,
    _Finalwillbeexercised,
    _Contraryintention,
    _Difference,
}
impl Field for ExpQty {
    fn tag(&self) -> u16 {
        return 983;
    }
}
pub struct ExpQty {
    value: f32,
}
impl Field for NoUnderlyingAmounts {
    fn tag(&self) -> u16 {
        return 984;
    }
}
pub struct NoUnderlyingAmounts {
    value: u16,
}
impl Field for UnderlyingPayAmount {
    fn tag(&self) -> u16 {
        return 985;
    }
}
pub struct UnderlyingPayAmount {
    value: f32,
}
impl Field for UnderlyingCollectAmount {
    fn tag(&self) -> u16 {
        return 986;
    }
}
pub struct UnderlyingCollectAmount {
    value: f32,
}
impl Field for UnderlyingSettlementDate {
    fn tag(&self) -> u16 {
        return 987;
    }
}
pub struct UnderlyingSettlementDate {
    value: u64,
}
impl Field for UnderlyingSettlementStatus {
    fn tag(&self) -> u16 {
        return 988;
    }
}
pub struct UnderlyingSettlementStatus {
    value: String,
}
impl Field for SecondaryIndividualAllocID {
    fn tag(&self) -> u16 {
        return 989;
    }
}
pub struct SecondaryIndividualAllocID {
    value: String,
}
impl Field for LegReportID {
    fn tag(&self) -> u16 {
        return 990;
    }
}
pub struct LegReportID {
    value: String,
}
impl Field for RndPx {
    fn tag(&self) -> u16 {
        return 991;
    }
}
pub struct RndPx {
    value: f32,
}
impl Field for IndividualAllocType {
    fn tag(&self) -> u16 {
        return 992;
    }
}
#[derive(Debug)]
pub enum IndividualAllocType {
    _Suballocate,
    _Thirdpartyallocation,
}
impl Field for AllocCustomerCapacity {
    fn tag(&self) -> u16 {
        return 993;
    }
}
pub struct AllocCustomerCapacity {
    value: String,
}
impl Field for TierCode {
    fn tag(&self) -> u16 {
        return 994;
    }
}
pub struct TierCode {
    value: String,
}
impl Field for UnitOfMeasure {
    fn tag(&self) -> u16 {
        return 996;
    }
}
#[derive(Debug)]
pub enum UnitOfMeasure {
    _Billioncubicfeet,
    _Millionbarrels,
    _Onemillionbtu,
    _Megawatthours,
    _Barrels,
    _Bushels,
    _Pounds,
    _Gallons,
    _Troyounces,
    _Metrictons,
    _Tons,
    _Usdollars,
}
impl Field for TimeUnit {
    fn tag(&self) -> u16 {
        return 997;
    }
}
#[derive(Debug)]
pub enum TimeUnit {
    _Hour,
    _Minute,
    _Second,
    _Day,
    _Week,
    _Month,
    _Year,
}
impl Field for UnderlyingUnitOfMeasure {
    fn tag(&self) -> u16 {
        return 998;
    }
}
pub struct UnderlyingUnitOfMeasure {
    value: String,
}
impl Field for LegUnitOfMeasure {
    fn tag(&self) -> u16 {
        return 999;
    }
}
pub struct LegUnitOfMeasure {
    value: String,
}
impl Field for UnderlyingTimeUnit {
    fn tag(&self) -> u16 {
        return 1000;
    }
}
pub struct UnderlyingTimeUnit {
    value: String,
}
impl Field for LegTimeUnit {
    fn tag(&self) -> u16 {
        return 1001;
    }
}
pub struct LegTimeUnit {
    value: String,
}
impl Field for AllocMethod {
    fn tag(&self) -> u16 {
        return 1002;
    }
}
#[derive(Debug)]
pub enum AllocMethod {
    _Automatic,
    _Guarantor,
    _Manual,
}
impl Field for TradeID {
    fn tag(&self) -> u16 {
        return 1003;
    }
}
pub struct TradeID {
    value: String,
}
impl Field for SideTradeReportID {
    fn tag(&self) -> u16 {
        return 1005;
    }
}
pub struct SideTradeReportID {
    value: String,
}
impl Field for SideFillStationCd {
    fn tag(&self) -> u16 {
        return 1006;
    }
}
pub struct SideFillStationCd {
    value: String,
}
impl Field for SideReasonCd {
    fn tag(&self) -> u16 {
        return 1007;
    }
}
pub struct SideReasonCd {
    value: String,
}
impl Field for SideTrdSubTyp {
    fn tag(&self) -> u16 {
        return 1008;
    }
}
pub struct SideTrdSubTyp {
    value: u16,
}
impl Field for SideQty {
    fn tag(&self) -> u16 {
        return 1009;
    }
}
pub struct SideQty {
    value: u16,
}
impl Field for MessageEventSource {
    fn tag(&self) -> u16 {
        return 1011;
    }
}
pub struct MessageEventSource {
    value: String,
}
impl Field for SideTrdRegTimestamp {
    fn tag(&self) -> u16 {
        return 1012;
    }
}
pub struct SideTrdRegTimestamp {
    value: u64,
}
impl Field for SideTrdRegTimestampType {
    fn tag(&self) -> u16 {
        return 1013;
    }
}
pub struct SideTrdRegTimestampType {
    value: u16,
}
impl Field for SideTrdRegTimestampSrc {
    fn tag(&self) -> u16 {
        return 1014;
    }
}
pub struct SideTrdRegTimestampSrc {
    value: String,
}
impl Field for AsOfIndicator {
    fn tag(&self) -> u16 {
        return 1015;
    }
}
#[derive(Debug)]
pub enum AsOfIndicator {
    _False,
    _True,
}
impl Field for NoSideTrdRegTS {
    fn tag(&self) -> u16 {
        return 1016;
    }
}
pub struct NoSideTrdRegTS {
    value: u16,
}
impl Field for LegOptionRatio {
    fn tag(&self) -> u16 {
        return 1017;
    }
}
pub struct LegOptionRatio {
    value: f32,
}
impl Field for NoInstrumentParties {
    fn tag(&self) -> u16 {
        return 1018;
    }
}
pub struct NoInstrumentParties {
    value: u16,
}
impl Field for InstrumentPartyID {
    fn tag(&self) -> u16 {
        return 1019;
    }
}
pub struct InstrumentPartyID {
    value: String,
}
impl Field for TradeVolume {
    fn tag(&self) -> u16 {
        return 1020;
    }
}
pub struct TradeVolume {
    value: f32,
}
impl Field for MDBookType {
    fn tag(&self) -> u16 {
        return 1021;
    }
}
#[derive(Debug)]
pub enum MDBookType {
    _Topofbook,
    _Pricedepth,
    _Orderdepth,
}
impl Field for MDFeedType {
    fn tag(&self) -> u16 {
        return 1022;
    }
}
pub struct MDFeedType {
    value: String,
}
impl Field for MDPriceLevel {
    fn tag(&self) -> u16 {
        return 1023;
    }
}
pub struct MDPriceLevel {
    value: u16,
}
impl Field for MDOriginType {
    fn tag(&self) -> u16 {
        return 1024;
    }
}
#[derive(Debug)]
pub enum MDOriginType {
    _Book,
    _Offbook,
    _Cross,
}
impl Field for FirstPx {
    fn tag(&self) -> u16 {
        return 1025;
    }
}
pub struct FirstPx {
    value: f32,
}
impl Field for MDEntrySpotRate {
    fn tag(&self) -> u16 {
        return 1026;
    }
}
pub struct MDEntrySpotRate {
    value: f32,
}
impl Field for MDEntryForwardPoints {
    fn tag(&self) -> u16 {
        return 1027;
    }
}
pub struct MDEntryForwardPoints {
    value: i8,
}
impl Field for ManualOrderIndicator {
    fn tag(&self) -> u16 {
        return 1028;
    }
}
pub struct ManualOrderIndicator {
    value: bool,
}
impl Field for CustDirectedOrder {
    fn tag(&self) -> u16 {
        return 1029;
    }
}
pub struct CustDirectedOrder {
    value: bool,
}
impl Field for ReceivedDeptID {
    fn tag(&self) -> u16 {
        return 1030;
    }
}
pub struct ReceivedDeptID {
    value: String,
}
impl Field for CustOrderHandlingInst {
    fn tag(&self) -> u16 {
        return 1031;
    }
}
#[derive(Debug)]
pub enum CustOrderHandlingInst {
    _Addonorder,
    _Allornone,
    _Cashnotheld,
    _Directedorder,
    _Exchangeforphysicaltransaction,
    _Fillorkill,
    _Imbalanceonly,
    _Immediateorcancel,
    _Limitonopen,
    _Limitonclose,
    _Marketatopen,
    _Marketatclose,
    _Marketonopen,
    _Marketonclose,
    _Minimumquantity,
    _Notheld,
    _Overtheday,
    _Pegged,
    _Reservesizeorder,
    _Stopstocktransaction,
    _Scale,
    _Timeorder,
    _Trailingstop,
    _Work,
}
impl Field for OrderHandlingInstSource {
    fn tag(&self) -> u16 {
        return 1032;
    }
}
#[derive(Debug)]
pub enum OrderHandlingInstSource {
    _Nasdoats,
}
impl Field for DeskType {
    fn tag(&self) -> u16 {
        return 1033;
    }
}
#[derive(Debug)]
pub enum DeskType {
    _Agency,
    _Arbitrage,
    _Derivatives,
    _International,
    _Institutional,
    _Other,
    _Preferredtrading,
    _Proprietary,
    _Programtrading,
    _Sales,
    _Trading,
}
impl Field for DeskTypeSource {
    fn tag(&self) -> u16 {
        return 1034;
    }
}
#[derive(Debug)]
pub enum DeskTypeSource {
    _Nasdoats,
}
impl Field for DeskOrderHandlingInst {
    fn tag(&self) -> u16 {
        return 1035;
    }
}
#[derive(Debug)]
pub enum DeskOrderHandlingInst {
    _Addonorder,
    _Allornone,
    _Cashnotheld,
    _Directedorder,
    _Exchangeforphysicaltransaction,
    _Fillorkill,
    _Imbalanceonly,
    _Immediateorcancel,
    _Limitonopen,
    _Limitonclose,
    _Marketatopen,
    _Marketatclose,
    _Marketonopen,
    _Marketonclose,
    _Minimumquantity,
    _Notheld,
    _Overtheday,
    _Pegged,
    _Reservesizeorder,
    _Stopstocktransaction,
    _Scale,
    _Timeorder,
    _Trailingstop,
    _Work,
}
impl Field for ExecAckStatus {
    fn tag(&self) -> u16 {
        return 1036;
    }
}
#[derive(Debug)]
pub enum ExecAckStatus {
    _Receivednotyetprocessed,
    _Accepted,
    _Dontknow,
}
impl Field for UnderlyingDeliveryAmount {
    fn tag(&self) -> u16 {
        return 1037;
    }
}
pub struct UnderlyingDeliveryAmount {
    value: f32,
}
impl Field for UnderlyingCapValue {
    fn tag(&self) -> u16 {
        return 1038;
    }
}
pub struct UnderlyingCapValue {
    value: f32,
}
impl Field for UnderlyingSettlMethod {
    fn tag(&self) -> u16 {
        return 1039;
    }
}
pub struct UnderlyingSettlMethod {
    value: String,
}
impl Field for SecondaryTradeID {
    fn tag(&self) -> u16 {
        return 1040;
    }
}
pub struct SecondaryTradeID {
    value: String,
}
impl Field for FirmTradeID {
    fn tag(&self) -> u16 {
        return 1041;
    }
}
pub struct FirmTradeID {
    value: String,
}
impl Field for SecondaryFirmTradeID {
    fn tag(&self) -> u16 {
        return 1042;
    }
}
pub struct SecondaryFirmTradeID {
    value: String,
}
impl Field for CollApplType {
    fn tag(&self) -> u16 {
        return 1043;
    }
}
#[derive(Debug)]
pub enum CollApplType {
    _Specificdeposit,
    _General,
}
impl Field for UnderlyingAdjustedQuantity {
    fn tag(&self) -> u16 {
        return 1044;
    }
}
pub struct UnderlyingAdjustedQuantity {
    value: f32,
}
impl Field for UnderlyingFXRate {
    fn tag(&self) -> u16 {
        return 1045;
    }
}
pub struct UnderlyingFXRate {
    value: f32,
}
impl Field for UnderlyingFXRateCalc {
    fn tag(&self) -> u16 {
        return 1046;
    }
}
#[derive(Debug)]
pub enum UnderlyingFXRateCalc {
    _Divide,
    _Multiply,
}
impl Field for AllocPositionEffect {
    fn tag(&self) -> u16 {
        return 1047;
    }
}
#[derive(Debug)]
pub enum AllocPositionEffect {
    _Open,
    _Close,
    _Rolled,
    _Fifo,
}
impl Field for DealingCapacity {
    fn tag(&self) -> u16 {
        return 1048;
    }
}
pub struct DealingCapacity {
    value: i8,
}
impl Field for InstrmtAssignmentMethod {
    fn tag(&self) -> u16 {
        return 1049;
    }
}
pub struct InstrmtAssignmentMethod {
    value: char,
}
impl Field for InstrumentPartyIDSource {
    fn tag(&self) -> u16 {
        return 1050;
    }
}
pub struct InstrumentPartyIDSource {
    value: char,
}
impl Field for InstrumentPartyRole {
    fn tag(&self) -> u16 {
        return 1051;
    }
}
pub struct InstrumentPartyRole {
    value: u16,
}
impl Field for NoInstrumentPartySubIDs {
    fn tag(&self) -> u16 {
        return 1052;
    }
}
pub struct NoInstrumentPartySubIDs {
    value: u16,
}
impl Field for InstrumentPartySubID {
    fn tag(&self) -> u16 {
        return 1053;
    }
}
pub struct InstrumentPartySubID {
    value: String,
}
impl Field for InstrumentPartySubIDType {
    fn tag(&self) -> u16 {
        return 1054;
    }
}
pub struct InstrumentPartySubIDType {
    value: u16,
}
impl Field for PositionCurrency {
    fn tag(&self) -> u16 {
        return 1055;
    }
}
pub struct PositionCurrency {
    value: String,
}
impl Field for CalculatedCcyLastQty {
    fn tag(&self) -> u16 {
        return 1056;
    }
}
pub struct CalculatedCcyLastQty {
    value: f32,
}
impl Field for AggressorIndicator {
    fn tag(&self) -> u16 {
        return 1057;
    }
}
#[derive(Debug)]
pub enum AggressorIndicator {
    _Yes,
    _No,
}
impl Field for NoUndlyInstrumentParties {
    fn tag(&self) -> u16 {
        return 1058;
    }
}
pub struct NoUndlyInstrumentParties {
    value: u16,
}
impl Field for UndlyInstrumentPartyID {
    fn tag(&self) -> u16 {
        return 1059;
    }
}
pub struct UndlyInstrumentPartyID {
    value: String,
}
impl Field for UndlyInstrumentPartyIDSource {
    fn tag(&self) -> u16 {
        return 1060;
    }
}
pub struct UndlyInstrumentPartyIDSource {
    value: char,
}
impl Field for UndlyInstrumentPartyRole {
    fn tag(&self) -> u16 {
        return 1061;
    }
}
pub struct UndlyInstrumentPartyRole {
    value: u16,
}
impl Field for NoUndlyInstrumentPartySubIDs {
    fn tag(&self) -> u16 {
        return 1062;
    }
}
pub struct NoUndlyInstrumentPartySubIDs {
    value: u16,
}
impl Field for UndlyInstrumentPartySubID {
    fn tag(&self) -> u16 {
        return 1063;
    }
}
pub struct UndlyInstrumentPartySubID {
    value: String,
}
impl Field for UndlyInstrumentPartySubIDType {
    fn tag(&self) -> u16 {
        return 1064;
    }
}
pub struct UndlyInstrumentPartySubIDType {
    value: u16,
}
impl Field for BidSwapPoints {
    fn tag(&self) -> u16 {
        return 1065;
    }
}
pub struct BidSwapPoints {
    value: i8,
}
impl Field for OfferSwapPoints {
    fn tag(&self) -> u16 {
        return 1066;
    }
}
pub struct OfferSwapPoints {
    value: i8,
}
impl Field for LegBidForwardPoints {
    fn tag(&self) -> u16 {
        return 1067;
    }
}
pub struct LegBidForwardPoints {
    value: i8,
}
impl Field for LegOfferForwardPoints {
    fn tag(&self) -> u16 {
        return 1068;
    }
}
pub struct LegOfferForwardPoints {
    value: i8,
}
impl Field for SwapPoints {
    fn tag(&self) -> u16 {
        return 1069;
    }
}
pub struct SwapPoints {
    value: i8,
}
impl Field for MDQuoteType {
    fn tag(&self) -> u16 {
        return 1070;
    }
}
#[derive(Debug)]
pub enum MDQuoteType {
    _Indicative,
    _Tradeable,
    _Restrictedtradeable,
    _Counter,
    _Indicativeandtradeable,
}
impl Field for LastSwapPoints {
    fn tag(&self) -> u16 {
        return 1071;
    }
}
pub struct LastSwapPoints {
    value: i8,
}
impl Field for SideGrossTradeAmt {
    fn tag(&self) -> u16 {
        return 1072;
    }
}
pub struct SideGrossTradeAmt {
    value: f32,
}
impl Field for LegLastForwardPoints {
    fn tag(&self) -> u16 {
        return 1073;
    }
}
pub struct LegLastForwardPoints {
    value: i8,
}
impl Field for LegCalculatedCcyLastQty {
    fn tag(&self) -> u16 {
        return 1074;
    }
}
pub struct LegCalculatedCcyLastQty {
    value: f32,
}
impl Field for LegGrossTradeAmt {
    fn tag(&self) -> u16 {
        return 1075;
    }
}
pub struct LegGrossTradeAmt {
    value: f32,
}
impl Field for MaturityTime {
    fn tag(&self) -> u16 {
        return 1079;
    }
}
pub struct MaturityTime {
    value: u64,
}
impl Field for RefOrderID {
    fn tag(&self) -> u16 {
        return 1080;
    }
}
pub struct RefOrderID {
    value: String,
}
impl Field for RefOrderIDSource {
    fn tag(&self) -> u16 {
        return 1081;
    }
}
#[derive(Debug)]
pub enum RefOrderIDSource {
    _Secondaryorderid,
    _Orderid,
    _Mdentryid,
    _Quoteentryid,
}
impl Field for SecondaryDisplayQty {
    fn tag(&self) -> u16 {
        return 1082;
    }
}
pub struct SecondaryDisplayQty {
    value: f32,
}
impl Field for DisplayWhen {
    fn tag(&self) -> u16 {
        return 1083;
    }
}
#[derive(Debug)]
pub enum DisplayWhen {
    _Immediate,
    _Exhaust,
}
impl Field for DisplayMethod {
    fn tag(&self) -> u16 {
        return 1084;
    }
}
#[derive(Debug)]
pub enum DisplayMethod {
    _Initial,
    _New,
    _Random,
}
impl Field for DisplayLowQty {
    fn tag(&self) -> u16 {
        return 1085;
    }
}
pub struct DisplayLowQty {
    value: f32,
}
impl Field for DisplayHighQty {
    fn tag(&self) -> u16 {
        return 1086;
    }
}
pub struct DisplayHighQty {
    value: f32,
}
impl Field for DisplayMinIncr {
    fn tag(&self) -> u16 {
        return 1087;
    }
}
pub struct DisplayMinIncr {
    value: f32,
}
impl Field for RefreshQty {
    fn tag(&self) -> u16 {
        return 1088;
    }
}
pub struct RefreshQty {
    value: f32,
}
impl Field for MatchIncrement {
    fn tag(&self) -> u16 {
        return 1089;
    }
}
pub struct MatchIncrement {
    value: f32,
}
impl Field for MaxPriceLevels {
    fn tag(&self) -> u16 {
        return 1090;
    }
}
pub struct MaxPriceLevels {
    value: u16,
}
impl Field for PreTradeAnonymity {
    fn tag(&self) -> u16 {
        return 1091;
    }
}
pub struct PreTradeAnonymity {
    value: bool,
}
impl Field for PriceProtectionScope {
    fn tag(&self) -> u16 {
        return 1092;
    }
}
#[derive(Debug)]
pub enum PriceProtectionScope {
    _None,
    _Local,
    _National,
    _Global,
}
impl Field for LotType {
    fn tag(&self) -> u16 {
        return 1093;
    }
}
#[derive(Debug)]
pub enum LotType {
    _Oddlot,
    _Roundlot,
    _Blocklot,
}
impl Field for PegPriceType {
    fn tag(&self) -> u16 {
        return 1094;
    }
}
#[derive(Debug)]
pub enum PegPriceType {
    _Lastpeg,
    _Midpricepeg,
    _Openingpeg,
    _Marketpeg,
    _Primarypeg,
    _Pegtovwap,
    _Trailingstoppeg,
    _Pegtolimitprice,
}
impl Field for PeggedRefPrice {
    fn tag(&self) -> u16 {
        return 1095;
    }
}
pub struct PeggedRefPrice {
    value: f32,
}
impl Field for PegSecurityIDSource {
    fn tag(&self) -> u16 {
        return 1096;
    }
}
pub struct PegSecurityIDSource {
    value: String,
}
impl Field for PegSecurityID {
    fn tag(&self) -> u16 {
        return 1097;
    }
}
pub struct PegSecurityID {
    value: String,
}
impl Field for PegSymbol {
    fn tag(&self) -> u16 {
        return 1098;
    }
}
pub struct PegSymbol {
    value: String,
}
impl Field for PegSecurityDesc {
    fn tag(&self) -> u16 {
        return 1099;
    }
}
pub struct PegSecurityDesc {
    value: String,
}
impl Field for TriggerType {
    fn tag(&self) -> u16 {
        return 1100;
    }
}
#[derive(Debug)]
pub enum TriggerType {
    _Partialexecution,
    _Specifiedtradingsession,
    _Nextauction,
    _Pricemovement,
}
impl Field for TriggerAction {
    fn tag(&self) -> u16 {
        return 1101;
    }
}
#[derive(Debug)]
pub enum TriggerAction {
    _Activate,
    _Modify,
    _Cancel,
}
impl Field for TriggerPrice {
    fn tag(&self) -> u16 {
        return 1102;
    }
}
pub struct TriggerPrice {
    value: f32,
}
impl Field for TriggerSymbol {
    fn tag(&self) -> u16 {
        return 1103;
    }
}
pub struct TriggerSymbol {
    value: String,
}
impl Field for TriggerSecurityID {
    fn tag(&self) -> u16 {
        return 1104;
    }
}
pub struct TriggerSecurityID {
    value: String,
}
impl Field for TriggerSecurityIDSource {
    fn tag(&self) -> u16 {
        return 1105;
    }
}
pub struct TriggerSecurityIDSource {
    value: String,
}
impl Field for TriggerSecurityDesc {
    fn tag(&self) -> u16 {
        return 1106;
    }
}
pub struct TriggerSecurityDesc {
    value: String,
}
impl Field for TriggerPriceType {
    fn tag(&self) -> u16 {
        return 1107;
    }
}
#[derive(Debug)]
pub enum TriggerPriceType {
    _Bestoffer,
    _Lasttrade,
    _Bestbid,
    _Bestbidorlasttrade,
    _Bestofferorlasttrade,
    _Bestmid,
}
impl Field for TriggerPriceTypeScope {
    fn tag(&self) -> u16 {
        return 1108;
    }
}
#[derive(Debug)]
pub enum TriggerPriceTypeScope {
    _None,
    _Local,
    _National,
    _Global,
}
impl Field for TriggerPriceDirection {
    fn tag(&self) -> u16 {
        return 1109;
    }
}
#[derive(Debug)]
pub enum TriggerPriceDirection {
    _Triggerifthepriceofthespecifiedtypegoesuptoorthroughthespecifiedtriggerprice,
    _Triggerifthepriceofthespecifiedtypegoesdowntoorthroughthespecifiedtriggerprice,
}
impl Field for TriggerNewPrice {
    fn tag(&self) -> u16 {
        return 1110;
    }
}
pub struct TriggerNewPrice {
    value: f32,
}
impl Field for TriggerOrderType {
    fn tag(&self) -> u16 {
        return 1111;
    }
}
#[derive(Debug)]
pub enum TriggerOrderType {
    _Market,
    _Limit,
}
impl Field for TriggerNewQty {
    fn tag(&self) -> u16 {
        return 1112;
    }
}
pub struct TriggerNewQty {
    value: f32,
}
impl Field for TriggerTradingSessionID {
    fn tag(&self) -> u16 {
        return 1113;
    }
}
pub struct TriggerTradingSessionID {
    value: String,
}
impl Field for TriggerTradingSessionSubID {
    fn tag(&self) -> u16 {
        return 1114;
    }
}
pub struct TriggerTradingSessionSubID {
    value: String,
}
impl Field for OrderCategory {
    fn tag(&self) -> u16 {
        return 1115;
    }
}
#[derive(Debug)]
pub enum OrderCategory {
    _Order,
    _Quote,
    _Privatelynegotiatedtrade,
    _Multilegorder,
    _Linkedorder,
    _Quoterequest,
    _Impliedorder,
    _Crossorder,
    _Streamingprice,
}
impl Field for NoRootPartyIDs {
    fn tag(&self) -> u16 {
        return 1116;
    }
}
pub struct NoRootPartyIDs {
    value: u16,
}
impl Field for RootPartyID {
    fn tag(&self) -> u16 {
        return 1117;
    }
}
pub struct RootPartyID {
    value: String,
}
impl Field for RootPartyIDSource {
    fn tag(&self) -> u16 {
        return 1118;
    }
}
pub struct RootPartyIDSource {
    value: char,
}
impl Field for RootPartyRole {
    fn tag(&self) -> u16 {
        return 1119;
    }
}
pub struct RootPartyRole {
    value: u16,
}
impl Field for NoRootPartySubIDs {
    fn tag(&self) -> u16 {
        return 1120;
    }
}
pub struct NoRootPartySubIDs {
    value: u16,
}
impl Field for RootPartySubID {
    fn tag(&self) -> u16 {
        return 1121;
    }
}
pub struct RootPartySubID {
    value: String,
}
impl Field for RootPartySubIDType {
    fn tag(&self) -> u16 {
        return 1122;
    }
}
pub struct RootPartySubIDType {
    value: u16,
}
impl Field for TradeHandlingInstr {
    fn tag(&self) -> u16 {
        return 1123;
    }
}
#[derive(Debug)]
pub enum TradeHandlingInstr {
    _Tradeconfirmation,
    _Twopartyreport,
    _Onepartyreportformatching,
    _Onepartyreportforpassthrough,
    _Automatedfloororderrouting,
    _Twopartyreportforclaim,
}
impl Field for OrigTradeHandlingInstr {
    fn tag(&self) -> u16 {
        return 1124;
    }
}
pub struct OrigTradeHandlingInstr {
    value: char,
}
impl Field for OrigTradeDate {
    fn tag(&self) -> u16 {
        return 1125;
    }
}
pub struct OrigTradeDate {
    value: u64,
}
impl Field for OrigTradeID {
    fn tag(&self) -> u16 {
        return 1126;
    }
}
pub struct OrigTradeID {
    value: String,
}
impl Field for OrigSecondaryTradeID {
    fn tag(&self) -> u16 {
        return 1127;
    }
}
pub struct OrigSecondaryTradeID {
    value: String,
}
impl Field for ApplVerID {
    fn tag(&self) -> u16 {
        return 1128;
    }
}
#[derive(Debug)]
pub enum ApplVerID {
    _Fix27,
    _Fix30,
    _Fix40,
    _Fix41,
    _Fix42,
    _Fix43,
    _Fix44,
    _Fix50,
    _Fix50Sp1,
}
impl Field for CstmApplVerID {
    fn tag(&self) -> u16 {
        return 1129;
    }
}
pub struct CstmApplVerID {
    value: String,
}
impl Field for RefApplVerID {
    fn tag(&self) -> u16 {
        return 1130;
    }
}
pub struct RefApplVerID {
    value: String,
}
impl Field for RefCstmApplVerID {
    fn tag(&self) -> u16 {
        return 1131;
    }
}
pub struct RefCstmApplVerID {
    value: String,
}
impl Field for TZTransactTime {
    fn tag(&self) -> u16 {
        return 1132;
    }
}
pub struct TZTransactTime {
    value: u64,
}
impl Field for ExDestinationIDSource {
    fn tag(&self) -> u16 {
        return 1133;
    }
}
#[derive(Debug)]
pub enum ExDestinationIDSource {
    _Bic,
    _Generallyacceptedmarketparticipantidentifier,
    _Proprietary,
    _Isocountrycode,
    _Mic,
}
impl Field for ReportedPxDiff {
    fn tag(&self) -> u16 {
        return 1134;
    }
}
pub struct ReportedPxDiff {
    value: bool,
}
impl Field for RptSys {
    fn tag(&self) -> u16 {
        return 1135;
    }
}
pub struct RptSys {
    value: String,
}
impl Field for AllocClearingFeeIndicator {
    fn tag(&self) -> u16 {
        return 1136;
    }
}
pub struct AllocClearingFeeIndicator {
    value: String,
}
impl Field for DefaultApplVerID {
    fn tag(&self) -> u16 {
        return 1137;
    }
}
pub struct DefaultApplVerID {
    value: String,
}
impl Field for DisplayQty {
    fn tag(&self) -> u16 {
        return 1138;
    }
}
pub struct DisplayQty {
    value: f32,
}
impl Field for ExchangeSpecialInstructions {
    fn tag(&self) -> u16 {
        return 1139;
    }
}
pub struct ExchangeSpecialInstructions {
    value: String,
}
impl Field for MaxTradeVol {
    fn tag(&self) -> u16 {
        return 1140;
    }
}
pub struct MaxTradeVol {
    value: f32,
}
impl Field for NoMDFeedTypes {
    fn tag(&self) -> u16 {
        return 1141;
    }
}
pub struct NoMDFeedTypes {
    value: u16,
}
impl Field for MatchAlgorithm {
    fn tag(&self) -> u16 {
        return 1142;
    }
}
pub struct MatchAlgorithm {
    value: String,
}
impl Field for MaxPriceVariation {
    fn tag(&self) -> u16 {
        return 1143;
    }
}
pub struct MaxPriceVariation {
    value: f32,
}
impl Field for ImpliedMarketIndicator {
    fn tag(&self) -> u16 {
        return 1144;
    }
}
#[derive(Debug)]
pub enum ImpliedMarketIndicator {
    _Notimplied,
    _Impliedin,
    _Impliedout,
    _Bothimpliedinandimpliedout,
}
impl Field for EventTime {
    fn tag(&self) -> u16 {
        return 1145;
    }
}
pub struct EventTime {
    value: u64,
}
impl Field for MinPriceIncrementAmount {
    fn tag(&self) -> u16 {
        return 1146;
    }
}
pub struct MinPriceIncrementAmount {
    value: f32,
}
impl Field for UnitOfMeasureQty {
    fn tag(&self) -> u16 {
        return 1147;
    }
}
pub struct UnitOfMeasureQty {
    value: f32,
}
impl Field for LowLimitPrice {
    fn tag(&self) -> u16 {
        return 1148;
    }
}
pub struct LowLimitPrice {
    value: f32,
}
impl Field for HighLimitPrice {
    fn tag(&self) -> u16 {
        return 1149;
    }
}
pub struct HighLimitPrice {
    value: f32,
}
impl Field for TradingReferencePrice {
    fn tag(&self) -> u16 {
        return 1150;
    }
}
pub struct TradingReferencePrice {
    value: f32,
}
impl Field for SecurityGroup {
    fn tag(&self) -> u16 {
        return 1151;
    }
}
pub struct SecurityGroup {
    value: String,
}
impl Field for LegNumber {
    fn tag(&self) -> u16 {
        return 1152;
    }
}
pub struct LegNumber {
    value: u16,
}
impl Field for SettlementCycleNo {
    fn tag(&self) -> u16 {
        return 1153;
    }
}
pub struct SettlementCycleNo {
    value: u16,
}
impl Field for SideCurrency {
    fn tag(&self) -> u16 {
        return 1154;
    }
}
pub struct SideCurrency {
    value: f32,
}
impl Field for SideSettlCurrency {
    fn tag(&self) -> u16 {
        return 1155;
    }
}
pub struct SideSettlCurrency {
    value: f32,
}
impl Field for ApplExtID {
    fn tag(&self) -> u16 {
        return 1156;
    }
}
pub struct ApplExtID {
    value: u16,
}
impl Field for CcyAmt {
    fn tag(&self) -> u16 {
        return 1157;
    }
}
pub struct CcyAmt {
    value: f32,
}
impl Field for NoSettlDetails {
    fn tag(&self) -> u16 {
        return 1158;
    }
}
pub struct NoSettlDetails {
    value: u16,
}
impl Field for SettlObligMode {
    fn tag(&self) -> u16 {
        return 1159;
    }
}
#[derive(Debug)]
pub enum SettlObligMode {
    _Preliminary,
    _Final,
}
impl Field for SettlObligMsgID {
    fn tag(&self) -> u16 {
        return 1160;
    }
}
pub struct SettlObligMsgID {
    value: String,
}
impl Field for SettlObligID {
    fn tag(&self) -> u16 {
        return 1161;
    }
}
pub struct SettlObligID {
    value: String,
}
impl Field for SettlObligTransType {
    fn tag(&self) -> u16 {
        return 1162;
    }
}
#[derive(Debug)]
pub enum SettlObligTransType {
    _Cancel,
    _New,
    _Replace,
    _Restate,
}
impl Field for SettlObligRefID {
    fn tag(&self) -> u16 {
        return 1163;
    }
}
pub struct SettlObligRefID {
    value: String,
}
impl Field for SettlObligSource {
    fn tag(&self) -> u16 {
        return 1164;
    }
}
#[derive(Debug)]
pub enum SettlObligSource {
    _Instructionsofbroker,
    _Instructionsforinstitution,
    _Investor,
}
impl Field for NoSettlOblig {
    fn tag(&self) -> u16 {
        return 1165;
    }
}
pub struct NoSettlOblig {
    value: u16,
}
impl Field for QuoteMsgID {
    fn tag(&self) -> u16 {
        return 1166;
    }
}
pub struct QuoteMsgID {
    value: String,
}
impl Field for QuoteEntryStatus {
    fn tag(&self) -> u16 {
        return 1167;
    }
}
#[derive(Debug)]
pub enum QuoteEntryStatus {
    _Accepted,
    _Rejected,
    _Removedfrommarket,
    _Expired,
    _Lockedmarketwarning,
    _Crossmarketwarning,
    _Canceledduetolockmarket,
    _Canceledduetocrossmarket,
    _Active,
}
impl Field for TotNoCxldQuotes {
    fn tag(&self) -> u16 {
        return 1168;
    }
}
pub struct TotNoCxldQuotes {
    value: u16,
}
impl Field for TotNoAccQuotes {
    fn tag(&self) -> u16 {
        return 1169;
    }
}
pub struct TotNoAccQuotes {
    value: u16,
}
impl Field for TotNoRejQuotes {
    fn tag(&self) -> u16 {
        return 1170;
    }
}
pub struct TotNoRejQuotes {
    value: u16,
}
impl Field for PrivateQuote {
    fn tag(&self) -> u16 {
        return 1171;
    }
}
pub struct PrivateQuote {
    value: bool,
}
impl Field for RespondentType {
    fn tag(&self) -> u16 {
        return 1172;
    }
}
#[derive(Debug)]
pub enum RespondentType {
    _Allmarketparticipants,
    _Specifiedmarketparticipants,
    _Allmarketmakers,
    _Primarymarketmaker,
}
impl Field for MDSubBookType {
    fn tag(&self) -> u16 {
        return 1173;
    }
}
pub struct MDSubBookType {
    value: u16,
}
impl Field for SecurityTradingEvent {
    fn tag(&self) -> u16 {
        return 1174;
    }
}
#[derive(Debug)]
pub enum SecurityTradingEvent {
    _Orderimbalanceauctionisextended,
    _Tradingresumes,
    _Pricevolatilityinterruption,
    _Changeoftradingsession,
    _Changeoftradingsubsession,
    _Changeofsecuritystatus,
    _Changeofbooktype,
    _Changeofmarketdepth,
}
impl Field for NoStatsIndicators {
    fn tag(&self) -> u16 {
        return 1175;
    }
}
pub struct NoStatsIndicators {
    value: u16,
}
impl Field for StatsType {
    fn tag(&self) -> u16 {
        return 1176;
    }
}
#[derive(Debug)]
pub enum StatsType {
    _Exchangelast,
    _High,
    _Averageprice,
    _Turnover,
}
impl Field for NoOfSecSizes {
    fn tag(&self) -> u16 {
        return 1177;
    }
}
pub struct NoOfSecSizes {
    value: u16,
}
impl Field for MDSecSizeType {
    fn tag(&self) -> u16 {
        return 1178;
    }
}
#[derive(Debug)]
pub enum MDSecSizeType {
    _Customer,
}
impl Field for MDSecSize {
    fn tag(&self) -> u16 {
        return 1179;
    }
}
pub struct MDSecSize {
    value: f32,
}
impl Field for ApplID {
    fn tag(&self) -> u16 {
        return 1180;
    }
}
pub struct ApplID {
    value: String,
}
impl Field for ApplSeqNum {
    fn tag(&self) -> u16 {
        return 1181;
    }
}
pub struct ApplSeqNum {
    value: u64,
}
impl Field for ApplBegSeqNum {
    fn tag(&self) -> u16 {
        return 1182;
    }
}
pub struct ApplBegSeqNum {
    value: u64,
}
impl Field for ApplEndSeqNum {
    fn tag(&self) -> u16 {
        return 1183;
    }
}
pub struct ApplEndSeqNum {
    value: u64,
}
impl Field for SecurityXMLLen {
    fn tag(&self) -> u16 {
        return 1184;
    }
}
pub struct SecurityXMLLen {
    value: usize,
}
impl Field for SecurityXML {
    fn tag(&self) -> u16 {
        return 1185;
    }
}
pub struct SecurityXML {
    value: String,
}
impl Field for SecurityXMLSchema {
    fn tag(&self) -> u16 {
        return 1186;
    }
}
pub struct SecurityXMLSchema {
    value: String,
}
impl Field for RefreshIndicator {
    fn tag(&self) -> u16 {
        return 1187;
    }
}
pub struct RefreshIndicator {
    value: bool,
}
impl Field for Volatility {
    fn tag(&self) -> u16 {
        return 1188;
    }
}
pub struct Volatility {
    value: f32,
}
impl Field for TimeToExpiration {
    fn tag(&self) -> u16 {
        return 1189;
    }
}
pub struct TimeToExpiration {
    value: f32,
}
impl Field for RiskFreeRate {
    fn tag(&self) -> u16 {
        return 1190;
    }
}
pub struct RiskFreeRate {
    value: f32,
}
impl Field for PriceUnitOfMeasure {
    fn tag(&self) -> u16 {
        return 1191;
    }
}
pub struct PriceUnitOfMeasure {
    value: String,
}
impl Field for PriceUnitOfMeasureQty {
    fn tag(&self) -> u16 {
        return 1192;
    }
}
pub struct PriceUnitOfMeasureQty {
    value: f32,
}
impl Field for SettlMethod {
    fn tag(&self) -> u16 {
        return 1193;
    }
}
#[derive(Debug)]
pub enum SettlMethod {
    _Cashsettlementrequired,
    _Physicalsettlementrequired,
}
impl Field for ExerciseStyle {
    fn tag(&self) -> u16 {
        return 1194;
    }
}
#[derive(Debug)]
pub enum ExerciseStyle {
    _European,
    _American,
    _Bermuda,
}
impl Field for OptPayAmount {
    fn tag(&self) -> u16 {
        return 1195;
    }
}
pub struct OptPayAmount {
    value: f32,
}
impl Field for PriceQuoteMethod {
    fn tag(&self) -> u16 {
        return 1196;
    }
}
#[derive(Debug)]
pub enum PriceQuoteMethod {
    _Standardmoneyperunitofaphysical,
    _Index,
    _Interestrateindex,
}
impl Field for FuturesValuationMethod {
    fn tag(&self) -> u16 {
        return 1197;
    }
}
#[derive(Debug)]
pub enum FuturesValuationMethod {
    _Premiumstyle,
    _Futuresstylemarktomarket,
    _Futuresstylewithanattachedcashadjustment,
}
impl Field for ListMethod {
    fn tag(&self) -> u16 {
        return 1198;
    }
}
#[derive(Debug)]
pub enum ListMethod {
    _Prelistedonly,
    _Userrequested,
}
impl Field for CapPrice {
    fn tag(&self) -> u16 {
        return 1199;
    }
}
pub struct CapPrice {
    value: f32,
}
impl Field for FloorPrice {
    fn tag(&self) -> u16 {
        return 1200;
    }
}
pub struct FloorPrice {
    value: f32,
}
impl Field for NoStrikeRules {
    fn tag(&self) -> u16 {
        return 1201;
    }
}
pub struct NoStrikeRules {
    value: u16,
}
impl Field for StartStrikePxRange {
    fn tag(&self) -> u16 {
        return 1202;
    }
}
pub struct StartStrikePxRange {
    value: f32,
}
impl Field for EndStrikePxRange {
    fn tag(&self) -> u16 {
        return 1203;
    }
}
pub struct EndStrikePxRange {
    value: f32,
}
impl Field for StrikeIncrement {
    fn tag(&self) -> u16 {
        return 1204;
    }
}
pub struct StrikeIncrement {
    value: f32,
}
impl Field for NoTickRules {
    fn tag(&self) -> u16 {
        return 1205;
    }
}
pub struct NoTickRules {
    value: u16,
}
impl Field for StartTickPriceRange {
    fn tag(&self) -> u16 {
        return 1206;
    }
}
pub struct StartTickPriceRange {
    value: f32,
}
impl Field for EndTickPriceRange {
    fn tag(&self) -> u16 {
        return 1207;
    }
}
pub struct EndTickPriceRange {
    value: f32,
}
impl Field for TickIncrement {
    fn tag(&self) -> u16 {
        return 1208;
    }
}
pub struct TickIncrement {
    value: f32,
}
impl Field for TickRuleType {
    fn tag(&self) -> u16 {
        return 1209;
    }
}
#[derive(Debug)]
pub enum TickRuleType {
    _Regular,
    _Variable,
    _Fixed,
    _Tradedasaspreadleg,
    _Settledasaspreadleg,
}
impl Field for NestedInstrAttribType {
    fn tag(&self) -> u16 {
        return 1210;
    }
}
pub struct NestedInstrAttribType {
    value: u16,
}
impl Field for NestedInstrAttribValue {
    fn tag(&self) -> u16 {
        return 1211;
    }
}
pub struct NestedInstrAttribValue {
    value: String,
}
impl Field for LegMaturityTime {
    fn tag(&self) -> u16 {
        return 1212;
    }
}
pub struct LegMaturityTime {
    value: u64,
}
impl Field for UnderlyingMaturityTime {
    fn tag(&self) -> u16 {
        return 1213;
    }
}
pub struct UnderlyingMaturityTime {
    value: u64,
}
impl Field for DerivativeSymbol {
    fn tag(&self) -> u16 {
        return 1214;
    }
}
pub struct DerivativeSymbol {
    value: String,
}
impl Field for DerivativeSymbolSfx {
    fn tag(&self) -> u16 {
        return 1215;
    }
}
pub struct DerivativeSymbolSfx {
    value: String,
}
impl Field for DerivativeSecurityID {
    fn tag(&self) -> u16 {
        return 1216;
    }
}
pub struct DerivativeSecurityID {
    value: String,
}
impl Field for DerivativeSecurityIDSource {
    fn tag(&self) -> u16 {
        return 1217;
    }
}
pub struct DerivativeSecurityIDSource {
    value: String,
}
impl Field for NoDerivativeSecurityAltID {
    fn tag(&self) -> u16 {
        return 1218;
    }
}
pub struct NoDerivativeSecurityAltID {
    value: u16,
}
impl Field for DerivativeSecurityAltID {
    fn tag(&self) -> u16 {
        return 1219;
    }
}
pub struct DerivativeSecurityAltID {
    value: String,
}
impl Field for DerivativeSecurityAltIDSource {
    fn tag(&self) -> u16 {
        return 1220;
    }
}
pub struct DerivativeSecurityAltIDSource {
    value: String,
}
impl Field for SecondaryLowLimitPrice {
    fn tag(&self) -> u16 {
        return 1221;
    }
}
pub struct SecondaryLowLimitPrice {
    value: f32,
}
impl Field for MaturityRuleID {
    fn tag(&self) -> u16 {
        return 1222;
    }
}
pub struct MaturityRuleID {
    value: String,
}
impl Field for StrikeRuleID {
    fn tag(&self) -> u16 {
        return 1223;
    }
}
pub struct StrikeRuleID {
    value: String,
}
impl Field for LegUnitOfMeasureQty {
    fn tag(&self) -> u16 {
        return 1224;
    }
}
pub struct LegUnitOfMeasureQty {
    value: f32,
}
impl Field for DerivativeOptPayAmount {
    fn tag(&self) -> u16 {
        return 1225;
    }
}
pub struct DerivativeOptPayAmount {
    value: f32,
}
impl Field for EndMaturityMonthYear {
    fn tag(&self) -> u16 {
        return 1226;
    }
}
pub struct EndMaturityMonthYear {
    value: u8,
}
impl Field for ProductComplex {
    fn tag(&self) -> u16 {
        return 1227;
    }
}
pub struct ProductComplex {
    value: String,
}
impl Field for DerivativeProductComplex {
    fn tag(&self) -> u16 {
        return 1228;
    }
}
pub struct DerivativeProductComplex {
    value: String,
}
impl Field for MaturityMonthYearIncrement {
    fn tag(&self) -> u16 {
        return 1229;
    }
}
pub struct MaturityMonthYearIncrement {
    value: u16,
}
impl Field for SecondaryHighLimitPrice {
    fn tag(&self) -> u16 {
        return 1230;
    }
}
pub struct SecondaryHighLimitPrice {
    value: f32,
}
impl Field for MinLotSize {
    fn tag(&self) -> u16 {
        return 1231;
    }
}
pub struct MinLotSize {
    value: f32,
}
impl Field for NoExecInstRules {
    fn tag(&self) -> u16 {
        return 1232;
    }
}
pub struct NoExecInstRules {
    value: u16,
}
impl Field for NoLotTypeRules {
    fn tag(&self) -> u16 {
        return 1234;
    }
}
pub struct NoLotTypeRules {
    value: u16,
}
impl Field for NoMatchRules {
    fn tag(&self) -> u16 {
        return 1235;
    }
}
pub struct NoMatchRules {
    value: u16,
}
impl Field for NoMaturityRules {
    fn tag(&self) -> u16 {
        return 1236;
    }
}
pub struct NoMaturityRules {
    value: u16,
}
impl Field for NoOrdTypeRules {
    fn tag(&self) -> u16 {
        return 1237;
    }
}
pub struct NoOrdTypeRules {
    value: u16,
}
impl Field for NoTimeInForceRules {
    fn tag(&self) -> u16 {
        return 1239;
    }
}
pub struct NoTimeInForceRules {
    value: u16,
}
impl Field for SecondaryTradingReferencePrice {
    fn tag(&self) -> u16 {
        return 1240;
    }
}
pub struct SecondaryTradingReferencePrice {
    value: f32,
}
impl Field for StartMaturityMonthYear {
    fn tag(&self) -> u16 {
        return 1241;
    }
}
pub struct StartMaturityMonthYear {
    value: u8,
}
impl Field for FlexProductEligibilityIndicator {
    fn tag(&self) -> u16 {
        return 1242;
    }
}
pub struct FlexProductEligibilityIndicator {
    value: bool,
}
impl Field for DerivFlexProductEligibilityIndicator {
    fn tag(&self) -> u16 {
        return 1243;
    }
}
pub struct DerivFlexProductEligibilityIndicator {
    value: bool,
}
impl Field for FlexibleIndicator {
    fn tag(&self) -> u16 {
        return 1244;
    }
}
pub struct FlexibleIndicator {
    value: bool,
}
impl Field for TradingCurrency {
    fn tag(&self) -> u16 {
        return 1245;
    }
}
pub struct TradingCurrency {
    value: f32,
}
impl Field for DerivativeProduct {
    fn tag(&self) -> u16 {
        return 1246;
    }
}
pub struct DerivativeProduct {
    value: u16,
}
impl Field for DerivativeSecurityGroup {
    fn tag(&self) -> u16 {
        return 1247;
    }
}
pub struct DerivativeSecurityGroup {
    value: String,
}
impl Field for DerivativeCFICode {
    fn tag(&self) -> u16 {
        return 1248;
    }
}
pub struct DerivativeCFICode {
    value: String,
}
impl Field for DerivativeSecurityType {
    fn tag(&self) -> u16 {
        return 1249;
    }
}
pub struct DerivativeSecurityType {
    value: String,
}
impl Field for DerivativeSecuritySubType {
    fn tag(&self) -> u16 {
        return 1250;
    }
}
pub struct DerivativeSecuritySubType {
    value: String,
}
impl Field for DerivativeMaturityMonthYear {
    fn tag(&self) -> u16 {
        return 1251;
    }
}
pub struct DerivativeMaturityMonthYear {
    value: u8,
}
impl Field for DerivativeMaturityDate {
    fn tag(&self) -> u16 {
        return 1252;
    }
}
pub struct DerivativeMaturityDate {
    value: u64,
}
impl Field for DerivativeMaturityTime {
    fn tag(&self) -> u16 {
        return 1253;
    }
}
pub struct DerivativeMaturityTime {
    value: u64,
}
impl Field for DerivativeSettleOnOpenFlag {
    fn tag(&self) -> u16 {
        return 1254;
    }
}
pub struct DerivativeSettleOnOpenFlag {
    value: String,
}
impl Field for DerivativeInstrmtAssignmentMethod {
    fn tag(&self) -> u16 {
        return 1255;
    }
}
pub struct DerivativeInstrmtAssignmentMethod {
    value: char,
}
impl Field for DerivativeSecurityStatus {
    fn tag(&self) -> u16 {
        return 1256;
    }
}
pub struct DerivativeSecurityStatus {
    value: String,
}
impl Field for DerivativeInstrRegistry {
    fn tag(&self) -> u16 {
        return 1257;
    }
}
pub struct DerivativeInstrRegistry {
    value: String,
}
impl Field for DerivativeCountryOfIssue {
    fn tag(&self) -> u16 {
        return 1258;
    }
}
pub struct DerivativeCountryOfIssue {
    value: String,
}
impl Field for DerivativeStateOrProvinceOfIssue {
    fn tag(&self) -> u16 {
        return 1259;
    }
}
pub struct DerivativeStateOrProvinceOfIssue {
    value: String,
}
impl Field for DerivativeLocaleOfIssue {
    fn tag(&self) -> u16 {
        return 1260;
    }
}
pub struct DerivativeLocaleOfIssue {
    value: String,
}
impl Field for DerivativeStrikePrice {
    fn tag(&self) -> u16 {
        return 1261;
    }
}
pub struct DerivativeStrikePrice {
    value: f32,
}
impl Field for DerivativeStrikeCurrency {
    fn tag(&self) -> u16 {
        return 1262;
    }
}
pub struct DerivativeStrikeCurrency {
    value: f32,
}
impl Field for DerivativeStrikeMultiplier {
    fn tag(&self) -> u16 {
        return 1263;
    }
}
pub struct DerivativeStrikeMultiplier {
    value: f32,
}
impl Field for DerivativeStrikeValue {
    fn tag(&self) -> u16 {
        return 1264;
    }
}
pub struct DerivativeStrikeValue {
    value: f32,
}
impl Field for DerivativeOptAttribute {
    fn tag(&self) -> u16 {
        return 1265;
    }
}
pub struct DerivativeOptAttribute {
    value: char,
}
impl Field for DerivativeContractMultiplier {
    fn tag(&self) -> u16 {
        return 1266;
    }
}
pub struct DerivativeContractMultiplier {
    value: f32,
}
impl Field for DerivativeMinPriceIncrement {
    fn tag(&self) -> u16 {
        return 1267;
    }
}
pub struct DerivativeMinPriceIncrement {
    value: f32,
}
impl Field for DerivativeMinPriceIncrementAmount {
    fn tag(&self) -> u16 {
        return 1268;
    }
}
pub struct DerivativeMinPriceIncrementAmount {
    value: f32,
}
impl Field for DerivativeUnitOfMeasure {
    fn tag(&self) -> u16 {
        return 1269;
    }
}
pub struct DerivativeUnitOfMeasure {
    value: String,
}
impl Field for DerivativeUnitOfMeasureQty {
    fn tag(&self) -> u16 {
        return 1270;
    }
}
pub struct DerivativeUnitOfMeasureQty {
    value: f32,
}
impl Field for DerivativeTimeUnit {
    fn tag(&self) -> u16 {
        return 1271;
    }
}
pub struct DerivativeTimeUnit {
    value: String,
}
impl Field for DerivativeSecurityExchange {
    fn tag(&self) -> u16 {
        return 1272;
    }
}
pub struct DerivativeSecurityExchange {
    value: String,
}
impl Field for DerivativePositionLimit {
    fn tag(&self) -> u16 {
        return 1273;
    }
}
pub struct DerivativePositionLimit {
    value: u16,
}
impl Field for DerivativeNTPositionLimit {
    fn tag(&self) -> u16 {
        return 1274;
    }
}
pub struct DerivativeNTPositionLimit {
    value: u16,
}
impl Field for DerivativeIssuer {
    fn tag(&self) -> u16 {
        return 1275;
    }
}
pub struct DerivativeIssuer {
    value: String,
}
impl Field for DerivativeIssueDate {
    fn tag(&self) -> u16 {
        return 1276;
    }
}
pub struct DerivativeIssueDate {
    value: u64,
}
impl Field for DerivativeEncodedIssuerLen {
    fn tag(&self) -> u16 {
        return 1277;
    }
}
pub struct DerivativeEncodedIssuerLen {
    value: usize,
}
impl Field for DerivativeEncodedIssuer {
    fn tag(&self) -> u16 {
        return 1278;
    }
}
pub struct DerivativeEncodedIssuer {
    value: [u8; 1024],
}
impl Field for DerivativeSecurityDesc {
    fn tag(&self) -> u16 {
        return 1279;
    }
}
pub struct DerivativeSecurityDesc {
    value: String,
}
impl Field for DerivativeEncodedSecurityDescLen {
    fn tag(&self) -> u16 {
        return 1280;
    }
}
pub struct DerivativeEncodedSecurityDescLen {
    value: usize,
}
impl Field for DerivativeEncodedSecurityDesc {
    fn tag(&self) -> u16 {
        return 1281;
    }
}
pub struct DerivativeEncodedSecurityDesc {
    value: [u8; 1024],
}
impl Field for DerivativeSecurityXMLLen {
    fn tag(&self) -> u16 {
        return 1282;
    }
}
pub struct DerivativeSecurityXMLLen {
    value: usize,
}
impl Field for DerivativeSecurityXML {
    fn tag(&self) -> u16 {
        return 1283;
    }
}
pub struct DerivativeSecurityXML {
    value: [u8; 1024],
}
impl Field for DerivativeSecurityXMLSchema {
    fn tag(&self) -> u16 {
        return 1284;
    }
}
pub struct DerivativeSecurityXMLSchema {
    value: String,
}
impl Field for DerivativeContractSettlMonth {
    fn tag(&self) -> u16 {
        return 1285;
    }
}
pub struct DerivativeContractSettlMonth {
    value: u8,
}
impl Field for NoDerivativeEvents {
    fn tag(&self) -> u16 {
        return 1286;
    }
}
pub struct NoDerivativeEvents {
    value: u16,
}
impl Field for DerivativeEventType {
    fn tag(&self) -> u16 {
        return 1287;
    }
}
pub struct DerivativeEventType {
    value: u16,
}
impl Field for DerivativeEventDate {
    fn tag(&self) -> u16 {
        return 1288;
    }
}
pub struct DerivativeEventDate {
    value: u64,
}
impl Field for DerivativeEventTime {
    fn tag(&self) -> u16 {
        return 1289;
    }
}
pub struct DerivativeEventTime {
    value: u64,
}
impl Field for DerivativeEventPx {
    fn tag(&self) -> u16 {
        return 1290;
    }
}
pub struct DerivativeEventPx {
    value: f32,
}
impl Field for DerivativeEventText {
    fn tag(&self) -> u16 {
        return 1291;
    }
}
pub struct DerivativeEventText {
    value: String,
}
impl Field for NoDerivativeInstrumentParties {
    fn tag(&self) -> u16 {
        return 1292;
    }
}
pub struct NoDerivativeInstrumentParties {
    value: u16,
}
impl Field for DerivativeInstrumentPartyID {
    fn tag(&self) -> u16 {
        return 1293;
    }
}
pub struct DerivativeInstrumentPartyID {
    value: String,
}
impl Field for DerivativeInstrumentPartyIDSource {
    fn tag(&self) -> u16 {
        return 1294;
    }
}
pub struct DerivativeInstrumentPartyIDSource {
    value: String,
}
impl Field for DerivativeInstrumentPartyRole {
    fn tag(&self) -> u16 {
        return 1295;
    }
}
pub struct DerivativeInstrumentPartyRole {
    value: u16,
}
impl Field for NoDerivativeInstrumentPartySubIDs {
    fn tag(&self) -> u16 {
        return 1296;
    }
}
pub struct NoDerivativeInstrumentPartySubIDs {
    value: u16,
}
impl Field for DerivativeInstrumentPartySubID {
    fn tag(&self) -> u16 {
        return 1297;
    }
}
pub struct DerivativeInstrumentPartySubID {
    value: String,
}
impl Field for DerivativeInstrumentPartySubIDType {
    fn tag(&self) -> u16 {
        return 1298;
    }
}
pub struct DerivativeInstrumentPartySubIDType {
    value: u16,
}
impl Field for DerivativeExerciseStyle {
    fn tag(&self) -> u16 {
        return 1299;
    }
}
pub struct DerivativeExerciseStyle {
    value: char,
}
impl Field for MarketSegmentID {
    fn tag(&self) -> u16 {
        return 1300;
    }
}
pub struct MarketSegmentID {
    value: String,
}
impl Field for MarketID {
    fn tag(&self) -> u16 {
        return 1301;
    }
}
pub struct MarketID {
    value: String,
}
impl Field for MaturityMonthYearIncrementUnits {
    fn tag(&self) -> u16 {
        return 1302;
    }
}
#[derive(Debug)]
pub enum MaturityMonthYearIncrementUnits {
    _Months,
    _Days,
    _Weeks,
    _Years,
}
impl Field for MaturityMonthYearFormat {
    fn tag(&self) -> u16 {
        return 1303;
    }
}
#[derive(Debug)]
pub enum MaturityMonthYearFormat {
    _Yearmonthonly,
    _Yearmonthday,
    _Yearmonthweek,
}
impl Field for StrikeExerciseStyle {
    fn tag(&self) -> u16 {
        return 1304;
    }
}
pub struct StrikeExerciseStyle {
    value: u16,
}
impl Field for SecondaryPriceLimitType {
    fn tag(&self) -> u16 {
        return 1305;
    }
}
pub struct SecondaryPriceLimitType {
    value: u16,
}
impl Field for PriceLimitType {
    fn tag(&self) -> u16 {
        return 1306;
    }
}
#[derive(Debug)]
pub enum PriceLimitType {
    _Price,
    _Ticks,
    _Percentage,
}
impl Field for ExecInstValue {
    fn tag(&self) -> u16 {
        return 1308;
    }
}
pub struct ExecInstValue {
    value: char,
}
impl Field for NoTradingSessionRules {
    fn tag(&self) -> u16 {
        return 1309;
    }
}
pub struct NoTradingSessionRules {
    value: u16,
}
impl Field for NoMarketSegments {
    fn tag(&self) -> u16 {
        return 1310;
    }
}
pub struct NoMarketSegments {
    value: u16,
}
impl Field for NoDerivativeInstrAttrib {
    fn tag(&self) -> u16 {
        return 1311;
    }
}
pub struct NoDerivativeInstrAttrib {
    value: u16,
}
impl Field for NoNestedInstrAttrib {
    fn tag(&self) -> u16 {
        return 1312;
    }
}
pub struct NoNestedInstrAttrib {
    value: u16,
}
impl Field for DerivativeInstrAttribType {
    fn tag(&self) -> u16 {
        return 1313;
    }
}
pub struct DerivativeInstrAttribType {
    value: u16,
}
impl Field for DerivativeInstrAttribValue {
    fn tag(&self) -> u16 {
        return 1314;
    }
}
pub struct DerivativeInstrAttribValue {
    value: String,
}
impl Field for DerivativePriceUnitOfMeasure {
    fn tag(&self) -> u16 {
        return 1315;
    }
}
pub struct DerivativePriceUnitOfMeasure {
    value: String,
}
impl Field for DerivativePriceUnitOfMeasureQty {
    fn tag(&self) -> u16 {
        return 1316;
    }
}
pub struct DerivativePriceUnitOfMeasureQty {
    value: f32,
}
impl Field for DerivativeSettlMethod {
    fn tag(&self) -> u16 {
        return 1317;
    }
}
pub struct DerivativeSettlMethod {
    value: char,
}
impl Field for DerivativePriceQuoteMethod {
    fn tag(&self) -> u16 {
        return 1318;
    }
}
pub struct DerivativePriceQuoteMethod {
    value: String,
}
impl Field for DerivativeFuturesValuationMethod {
    fn tag(&self) -> u16 {
        return 1319;
    }
}
pub struct DerivativeFuturesValuationMethod {
    value: String,
}
impl Field for DerivativeListMethod {
    fn tag(&self) -> u16 {
        return 1320;
    }
}
pub struct DerivativeListMethod {
    value: u16,
}
impl Field for DerivativeCapPrice {
    fn tag(&self) -> u16 {
        return 1321;
    }
}
pub struct DerivativeCapPrice {
    value: f32,
}
impl Field for DerivativeFloorPrice {
    fn tag(&self) -> u16 {
        return 1322;
    }
}
pub struct DerivativeFloorPrice {
    value: f32,
}
impl Field for DerivativePutOrCall {
    fn tag(&self) -> u16 {
        return 1323;
    }
}
pub struct DerivativePutOrCall {
    value: u16,
}
impl Field for ListUpdateAction {
    fn tag(&self) -> u16 {
        return 1324;
    }
}
pub struct ListUpdateAction {
    value: char,
}
impl Field for ParentMktSegmID {
    fn tag(&self) -> u16 {
        return 1325;
    }
}
pub struct ParentMktSegmID {
    value: String,
}
impl Field for TradingSessionDesc {
    fn tag(&self) -> u16 {
        return 1326;
    }
}
pub struct TradingSessionDesc {
    value: String,
}
impl Field for TradSesUpdateAction {
    fn tag(&self) -> u16 {
        return 1327;
    }
}
pub struct TradSesUpdateAction {
    value: char,
}
impl Field for RejectText {
    fn tag(&self) -> u16 {
        return 1328;
    }
}
pub struct RejectText {
    value: String,
}
impl Field for FeeMultiplier {
    fn tag(&self) -> u16 {
        return 1329;
    }
}
pub struct FeeMultiplier {
    value: f32,
}
impl Field for UnderlyingLegSymbol {
    fn tag(&self) -> u16 {
        return 1330;
    }
}
pub struct UnderlyingLegSymbol {
    value: String,
}
impl Field for UnderlyingLegSymbolSfx {
    fn tag(&self) -> u16 {
        return 1331;
    }
}
pub struct UnderlyingLegSymbolSfx {
    value: String,
}
impl Field for UnderlyingLegSecurityID {
    fn tag(&self) -> u16 {
        return 1332;
    }
}
pub struct UnderlyingLegSecurityID {
    value: String,
}
impl Field for UnderlyingLegSecurityIDSource {
    fn tag(&self) -> u16 {
        return 1333;
    }
}
pub struct UnderlyingLegSecurityIDSource {
    value: String,
}
impl Field for NoUnderlyingLegSecurityAltID {
    fn tag(&self) -> u16 {
        return 1334;
    }
}
pub struct NoUnderlyingLegSecurityAltID {
    value: u16,
}
impl Field for UnderlyingLegSecurityAltID {
    fn tag(&self) -> u16 {
        return 1335;
    }
}
pub struct UnderlyingLegSecurityAltID {
    value: String,
}
impl Field for UnderlyingLegSecurityAltIDSource {
    fn tag(&self) -> u16 {
        return 1336;
    }
}
pub struct UnderlyingLegSecurityAltIDSource {
    value: String,
}
impl Field for UnderlyingLegSecurityType {
    fn tag(&self) -> u16 {
        return 1337;
    }
}
pub struct UnderlyingLegSecurityType {
    value: String,
}
impl Field for UnderlyingLegSecuritySubType {
    fn tag(&self) -> u16 {
        return 1338;
    }
}
pub struct UnderlyingLegSecuritySubType {
    value: String,
}
impl Field for UnderlyingLegMaturityMonthYear {
    fn tag(&self) -> u16 {
        return 1339;
    }
}
pub struct UnderlyingLegMaturityMonthYear {
    value: u8,
}
impl Field for UnderlyingLegStrikePrice {
    fn tag(&self) -> u16 {
        return 1340;
    }
}
pub struct UnderlyingLegStrikePrice {
    value: f32,
}
impl Field for UnderlyingLegSecurityExchange {
    fn tag(&self) -> u16 {
        return 1341;
    }
}
pub struct UnderlyingLegSecurityExchange {
    value: String,
}
impl Field for NoOfLegUnderlyings {
    fn tag(&self) -> u16 {
        return 1342;
    }
}
pub struct NoOfLegUnderlyings {
    value: u16,
}
impl Field for UnderlyingLegPutOrCall {
    fn tag(&self) -> u16 {
        return 1343;
    }
}
pub struct UnderlyingLegPutOrCall {
    value: u16,
}
impl Field for UnderlyingLegCFICode {
    fn tag(&self) -> u16 {
        return 1344;
    }
}
pub struct UnderlyingLegCFICode {
    value: String,
}
impl Field for UnderlyingLegMaturityDate {
    fn tag(&self) -> u16 {
        return 1345;
    }
}
pub struct UnderlyingLegMaturityDate {
    value: u64,
}
impl Field for ApplReqID {
    fn tag(&self) -> u16 {
        return 1346;
    }
}
pub struct ApplReqID {
    value: String,
}
impl Field for ApplReqType {
    fn tag(&self) -> u16 {
        return 1347;
    }
}
#[derive(Debug)]
pub enum ApplReqType {
    _Retransmissionofapplicationmessagesforthespecifiedapplications,
    _Subscriptiontothespecifiedapplications,
    _Requestforthelastappllastseqnumpublishedforthespecifiedapplications,
    _Requestvalidsetofapplications,
    _Unsubscribetothespecifiedapplications,
}
impl Field for ApplResponseType {
    fn tag(&self) -> u16 {
        return 1348;
    }
}
#[derive(Debug)]
pub enum ApplResponseType {
    _Requestsuccessfullyprocessed,
    _Applicationdoesnotexist,
    _Messagesnotavailable,
}
impl Field for ApplTotalMessageCount {
    fn tag(&self) -> u16 {
        return 1349;
    }
}
pub struct ApplTotalMessageCount {
    value: u16,
}
impl Field for ApplLastSeqNum {
    fn tag(&self) -> u16 {
        return 1350;
    }
}
pub struct ApplLastSeqNum {
    value: u64,
}
impl Field for NoApplIDs {
    fn tag(&self) -> u16 {
        return 1351;
    }
}
pub struct NoApplIDs {
    value: u16,
}
impl Field for ApplResendFlag {
    fn tag(&self) -> u16 {
        return 1352;
    }
}
pub struct ApplResendFlag {
    value: bool,
}
impl Field for ApplResponseID {
    fn tag(&self) -> u16 {
        return 1353;
    }
}
pub struct ApplResponseID {
    value: String,
}
impl Field for ApplResponseError {
    fn tag(&self) -> u16 {
        return 1354;
    }
}
#[derive(Debug)]
pub enum ApplResponseError {
    _Applicationdoesnotexist,
    _Messagesrequestedarenotavailable,
    _Usernotauthorizedforapplication,
}
impl Field for RefApplID {
    fn tag(&self) -> u16 {
        return 1355;
    }
}
pub struct RefApplID {
    value: String,
}
impl Field for ApplReportID {
    fn tag(&self) -> u16 {
        return 1356;
    }
}
pub struct ApplReportID {
    value: String,
}
impl Field for RefApplLastSeqNum {
    fn tag(&self) -> u16 {
        return 1357;
    }
}
pub struct RefApplLastSeqNum {
    value: u64,
}
impl Field for LegPutOrCall {
    fn tag(&self) -> u16 {
        return 1358;
    }
}
pub struct LegPutOrCall {
    value: u16,
}
impl Field for TotNoFills {
    fn tag(&self) -> u16 {
        return 1361;
    }
}
pub struct TotNoFills {
    value: u16,
}
impl Field for NoFills {
    fn tag(&self) -> u16 {
        return 1362;
    }
}
pub struct NoFills {
    value: u16,
}
impl Field for FillExecID {
    fn tag(&self) -> u16 {
        return 1363;
    }
}
pub struct FillExecID {
    value: String,
}
impl Field for FillPx {
    fn tag(&self) -> u16 {
        return 1364;
    }
}
pub struct FillPx {
    value: f32,
}
impl Field for FillQty {
    fn tag(&self) -> u16 {
        return 1365;
    }
}
pub struct FillQty {
    value: f32,
}
impl Field for LegAllocID {
    fn tag(&self) -> u16 {
        return 1366;
    }
}
pub struct LegAllocID {
    value: String,
}
impl Field for LegAllocSettlCurrency {
    fn tag(&self) -> u16 {
        return 1367;
    }
}
pub struct LegAllocSettlCurrency {
    value: f32,
}
impl Field for TradSesEvent {
    fn tag(&self) -> u16 {
        return 1368;
    }
}
#[derive(Debug)]
pub enum TradSesEvent {
    _Tradingresumes,
    _Changeoftradingsession,
    _Changeoftradingsubsession,
    _Changeoftradingstatus,
}
impl Field for MassActionReportID {
    fn tag(&self) -> u16 {
        return 1369;
    }
}
pub struct MassActionReportID {
    value: String,
}
impl Field for NoNotAffectedOrders {
    fn tag(&self) -> u16 {
        return 1370;
    }
}
pub struct NoNotAffectedOrders {
    value: u16,
}
impl Field for NotAffectedOrderID {
    fn tag(&self) -> u16 {
        return 1371;
    }
}
pub struct NotAffectedOrderID {
    value: String,
}
impl Field for NotAffOrigClOrdID {
    fn tag(&self) -> u16 {
        return 1372;
    }
}
pub struct NotAffOrigClOrdID {
    value: String,
}
impl Field for MassActionType {
    fn tag(&self) -> u16 {
        return 1373;
    }
}
#[derive(Debug)]
pub enum MassActionType {
    _Suspendorders,
    _Releaseordersfromsuspension,
    _Cancelorders,
}
impl Field for MassActionScope {
    fn tag(&self) -> u16 {
        return 1374;
    }
}
#[derive(Debug)]
pub enum MassActionScope {
    _Allordersforasecurity,
    _Allordersforanunderlyingsecurity,
    _Allordersforaproduct,
    _Allordersforacficode,
    _Allordersforasecuritytype,
    _Allordersforatradingsession,
    _Allorders,
    _Allordersforamarket,
    _Allordersforamarketsegment,
    _Allordersforasecuritygroup,
}
impl Field for MassActionResponse {
    fn tag(&self) -> u16 {
        return 1375;
    }
}
#[derive(Debug)]
pub enum MassActionResponse {
    _Rejected,
    _Accepted,
}
impl Field for MassActionRejectReason {
    fn tag(&self) -> u16 {
        return 1376;
    }
}
#[derive(Debug)]
pub enum MassActionRejectReason {
    _Massactionnotsupported,
    _Invalidorunknownsecurity,
    _Invalidorunknownunderlyingsecurity,
    _Invalidorunknownproduct,
    _Invalidorunknowncficode,
    _Invalidorunknownsecuritytype,
    _Invalidorunknowntradingsession,
    _Invalidorunknownmarket,
    _Invalidorunknownmarketsegment,
    _Invalidorunknownsecuritygroup,
    _Other,
}
impl Field for MultilegModel {
    fn tag(&self) -> u16 {
        return 1377;
    }
}
#[derive(Debug)]
pub enum MultilegModel {
    _Predefinedmultilegsecurity,
    _Userdefinedmultlegsecurity,
    _Userdefinednonsecuritizedmultileg,
}
impl Field for MultilegPriceMethod {
    fn tag(&self) -> u16 {
        return 1378;
    }
}
#[derive(Debug)]
pub enum MultilegPriceMethod {
    _Netprice,
    _Reversednetprice,
    _Yielddifference,
    _Individual,
    _Contractweightedaverageprice,
    _Multipliedprice,
}
impl Field for LegVolatility {
    fn tag(&self) -> u16 {
        return 1379;
    }
}
pub struct LegVolatility {
    value: f32,
}
impl Field for DividendYield {
    fn tag(&self) -> u16 {
        return 1380;
    }
}
pub struct DividendYield {
    value: f32,
}
impl Field for LegDividendYield {
    fn tag(&self) -> u16 {
        return 1381;
    }
}
pub struct LegDividendYield {
    value: f32,
}
impl Field for CurrencyRatio {
    fn tag(&self) -> u16 {
        return 1382;
    }
}
pub struct CurrencyRatio {
    value: f32,
}
impl Field for LegCurrencyRatio {
    fn tag(&self) -> u16 {
        return 1383;
    }
}
pub struct LegCurrencyRatio {
    value: f32,
}
impl Field for LegExecInst {
    fn tag(&self) -> u16 {
        return 1384;
    }
}
pub struct LegExecInst {
    value: String,
}
impl Field for ContingencyType {
    fn tag(&self) -> u16 {
        return 1385;
    }
}
#[derive(Debug)]
pub enum ContingencyType {
    _Onecancelstheother,
    _Onetriggerstheother,
    _Oneupdatestheother3,
    _Oneupdatestheother4,
}
impl Field for ListRejectReason {
    fn tag(&self) -> u16 {
        return 1386;
    }
}
#[derive(Debug)]
pub enum ListRejectReason {
    _Broker,
    _Exchangeclosed,
    _Toolatetoenter,
    _Unknownorder,
    _Duplicateorder,
    _Unsupportedordercharacteristic,
    _Other,
}
impl Field for NoTrdRepIndicators {
    fn tag(&self) -> u16 {
        return 1387;
    }
}
pub struct NoTrdRepIndicators {
    value: u16,
}
impl Field for TrdRepPartyRole {
    fn tag(&self) -> u16 {
        return 1388;
    }
}
pub struct TrdRepPartyRole {
    value: u16,
}
impl Field for TrdRepIndicator {
    fn tag(&self) -> u16 {
        return 1389;
    }
}
pub struct TrdRepIndicator {
    value: bool,
}
impl Field for TradePublishIndicator {
    fn tag(&self) -> u16 {
        return 1390;
    }
}
#[derive(Debug)]
pub enum TradePublishIndicator {
    _Donotpublishtrade,
    _Publishtrade,
    _Deferredpublication,
}
impl Field for UnderlyingLegOptAttribute {
    fn tag(&self) -> u16 {
        return 1391;
    }
}
pub struct UnderlyingLegOptAttribute {
    value: char,
}
impl Field for UnderlyingLegSecurityDesc {
    fn tag(&self) -> u16 {
        return 1392;
    }
}
pub struct UnderlyingLegSecurityDesc {
    value: String,
}
impl Field for MarketReqID {
    fn tag(&self) -> u16 {
        return 1393;
    }
}
pub struct MarketReqID {
    value: String,
}
impl Field for MarketReportID {
    fn tag(&self) -> u16 {
        return 1394;
    }
}
pub struct MarketReportID {
    value: String,
}
impl Field for MarketUpdateAction {
    fn tag(&self) -> u16 {
        return 1395;
    }
}
#[derive(Debug)]
pub enum MarketUpdateAction {
    _Add,
    _Delete,
    _Modify,
}
impl Field for MarketSegmentDesc {
    fn tag(&self) -> u16 {
        return 1396;
    }
}
pub struct MarketSegmentDesc {
    value: String,
}
impl Field for EncodedMktSegmDescLen {
    fn tag(&self) -> u16 {
        return 1397;
    }
}
pub struct EncodedMktSegmDescLen {
    value: usize,
}
impl Field for EncodedMktSegmDesc {
    fn tag(&self) -> u16 {
        return 1398;
    }
}
pub struct EncodedMktSegmDesc {
    value: [u8; 1024],
}
impl Field for ApplNewSeqNum {
    fn tag(&self) -> u16 {
        return 1399;
    }
}
pub struct ApplNewSeqNum {
    value: u64,
}
impl Field for EncryptedPasswordMethod {
    fn tag(&self) -> u16 {
        return 1400;
    }
}
pub struct EncryptedPasswordMethod {
    value: u16,
}
impl Field for EncryptedPasswordLen {
    fn tag(&self) -> u16 {
        return 1401;
    }
}
pub struct EncryptedPasswordLen {
    value: usize,
}
impl Field for EncryptedPassword {
    fn tag(&self) -> u16 {
        return 1402;
    }
}
pub struct EncryptedPassword {
    value: [u8; 1024],
}
impl Field for EncryptedNewPasswordLen {
    fn tag(&self) -> u16 {
        return 1403;
    }
}
pub struct EncryptedNewPasswordLen {
    value: usize,
}
impl Field for EncryptedNewPassword {
    fn tag(&self) -> u16 {
        return 1404;
    }
}
pub struct EncryptedNewPassword {
    value: [u8; 1024],
}
impl Field for UnderlyingLegMaturityTime {
    fn tag(&self) -> u16 {
        return 1405;
    }
}
pub struct UnderlyingLegMaturityTime {
    value: u64,
}
impl Field for RefApplExtID {
    fn tag(&self) -> u16 {
        return 1406;
    }
}
pub struct RefApplExtID {
    value: u16,
}
impl Field for DefaultApplExtID {
    fn tag(&self) -> u16 {
        return 1407;
    }
}
pub struct DefaultApplExtID {
    value: u16,
}
impl Field for DefaultCstmApplVerID {
    fn tag(&self) -> u16 {
        return 1408;
    }
}
pub struct DefaultCstmApplVerID {
    value: String,
}
impl Field for SessionStatus {
    fn tag(&self) -> u16 {
        return 1409;
    }
}
#[derive(Debug)]
pub enum SessionStatus {
    _Sessionactive,
    _Sessionpasswordchanged,
    _Sessionpasswordduetoexpire,
    _Newsessionpassworddoesnotcomplywithpolicy,
    _Sessionlogoutcomplete,
    _Invalidusernameorpassword,
    _Accountlocked,
    _Logonsarenotallowedatthistime,
    _Passwordexpired,
}
impl Field for DefaultVerIndicator {
    fn tag(&self) -> u16 {
        return 1410;
    }
}
pub struct DefaultVerIndicator {
    value: bool,
}
impl Field for Nested4PartySubIDType {
    fn tag(&self) -> u16 {
        return 1411;
    }
}
pub struct Nested4PartySubIDType {
    value: u16,
}
impl Field for Nested4PartySubID {
    fn tag(&self) -> u16 {
        return 1412;
    }
}
pub struct Nested4PartySubID {
    value: String,
}
impl Field for NoNested4PartySubIDs {
    fn tag(&self) -> u16 {
        return 1413;
    }
}
pub struct NoNested4PartySubIDs {
    value: u16,
}
impl Field for NoNested4PartyIDs {
    fn tag(&self) -> u16 {
        return 1414;
    }
}
pub struct NoNested4PartyIDs {
    value: u16,
}
impl Field for Nested4PartyID {
    fn tag(&self) -> u16 {
        return 1415;
    }
}
pub struct Nested4PartyID {
    value: String,
}
impl Field for Nested4PartyIDSource {
    fn tag(&self) -> u16 {
        return 1416;
    }
}
pub struct Nested4PartyIDSource {
    value: char,
}
impl Field for Nested4PartyRole {
    fn tag(&self) -> u16 {
        return 1417;
    }
}
pub struct Nested4PartyRole {
    value: u16,
}
impl Field for LegLastQty {
    fn tag(&self) -> u16 {
        return 1418;
    }
}
pub struct LegLastQty {
    value: f32,
}
impl Field for UnderlyingExerciseStyle {
    fn tag(&self) -> u16 {
        return 1419;
    }
}
pub struct UnderlyingExerciseStyle {
    value: u16,
}
impl Field for LegExerciseStyle {
    fn tag(&self) -> u16 {
        return 1420;
    }
}
pub struct LegExerciseStyle {
    value: u16,
}
impl Field for LegPriceUnitOfMeasure {
    fn tag(&self) -> u16 {
        return 1421;
    }
}
pub struct LegPriceUnitOfMeasure {
    value: String,
}
impl Field for LegPriceUnitOfMeasureQty {
    fn tag(&self) -> u16 {
        return 1422;
    }
}
pub struct LegPriceUnitOfMeasureQty {
    value: f32,
}
impl Field for UnderlyingUnitOfMeasureQty {
    fn tag(&self) -> u16 {
        return 1423;
    }
}
pub struct UnderlyingUnitOfMeasureQty {
    value: f32,
}
impl Field for UnderlyingPriceUnitOfMeasure {
    fn tag(&self) -> u16 {
        return 1424;
    }
}
pub struct UnderlyingPriceUnitOfMeasure {
    value: String,
}
impl Field for UnderlyingPriceUnitOfMeasureQty {
    fn tag(&self) -> u16 {
        return 1425;
    }
}
pub struct UnderlyingPriceUnitOfMeasureQty {
    value: f32,
}
impl Field for ApplReportType {
    fn tag(&self) -> u16 {
        return 1426;
    }
}
#[derive(Debug)]
pub enum ApplReportType {
    _Resetapplseqnumtonewvaluespecifiedinapplnewseqnum,
    _Reportsthatthelastmessagehasbeensentfortheapplidsrefertorefappllastseqnum,
    _Heartbeatmessageindicatingthatapplicationidentifiedbyrefapplid,
}
