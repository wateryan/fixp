use fix_4_2_0::fields::*;
struct HeartbeatMessage {
    test_req_id: Option<TestReqID>,
}
struct TestRequestMessage {
    test_req_id: TestReqID,
}
struct ResendRequestMessage {
    begin_seq_no: BeginSeqNo,
    end_seq_no: EndSeqNo,
}
struct RejectMessage {
    ref_seq_num: RefSeqNum,
    ref_tag_id: Option<RefTagID>,
    ref_msg_type: Option<RefMsgType>,
    session_reject_reason: Option<SessionRejectReason>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct SequenceResetMessage {
    gap_fill_flag: Option<GapFillFlag>,
    new_seq_no: NewSeqNo,
}
struct LogoutMessage {
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct IndicationofInterestMessage {
    i_o_iid: IOIid,
    i_oi_trans_type: IOITransType,
    i_oi_ref_id: Option<IOIRefID>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    side: Side,
    i_oi_shares: IOIShares,
    price: Option<Price>,
    currency: Option<Currency>,
    valid_until_time: Option<ValidUntilTime>,
    i_oi_qlty_ind: Option<IOIQltyInd>,
    i_oi_natural_flag: Option<IOINaturalFlag>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
    transact_time: Option<TransactTime>,
    u_rl_link: Option<URLLink>,
    spread_to_benchmark: Option<SpreadToBenchmark>,
    benchmark: Option<Benchmark>,
}
struct AdvertisementMessage {
    adv_id: AdvId,
    adv_trans_type: AdvTransType,
    adv_ref_id: Option<AdvRefID>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    adv_side: AdvSide,
    shares: Shares,
    price: Option<Price>,
    currency: Option<Currency>,
    trade_date: Option<TradeDate>,
    transact_time: Option<TransactTime>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
    u_rl_link: Option<URLLink>,
    last_mkt: Option<LastMkt>,
    trading_session_id: Option<TradingSessionID>,
}
struct ExecutionReportMessage {
    order_id: OrderID,
    secondary_order_id: Option<SecondaryOrderID>,
    cl_ord_id: Option<ClOrdID>,
    orig_cl_ord_id: Option<OrigClOrdID>,
    client_id: Option<ClientID>,
    exec_broker: Option<ExecBroker>,
    list_id: Option<ListID>,
    exec_id: ExecID,
    exec_trans_type: ExecTransType,
    exec_ref_id: Option<ExecRefID>,
    exec_type: ExecType,
    ord_status: OrdStatus,
    ord_rej_reason: Option<OrdRejReason>,
    exec_restatement_reason: Option<ExecRestatementReason>,
    account: Option<Account>,
    settlmnt_typ: Option<SettlmntTyp>,
    fut_sett_date: Option<FutSettDate>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    side: Side,
    order_qty: Option<OrderQty>,
    cash_order_qty: Option<CashOrderQty>,
    ord_type: Option<OrdType>,
    price: Option<Price>,
    stop_px: Option<StopPx>,
    peg_difference: Option<PegDifference>,
    discretion_inst: Option<DiscretionInst>,
    discretion_offset: Option<DiscretionOffset>,
    currency: Option<Currency>,
    compliance_id: Option<ComplianceID>,
    solicited_flag: Option<SolicitedFlag>,
    time_in_force: Option<TimeInForce>,
    effective_time: Option<EffectiveTime>,
    expire_date: Option<ExpireDate>,
    expire_time: Option<ExpireTime>,
    exec_inst: Option<ExecInst>,
    rule80_a: Option<Rule80A>,
    last_shares: Option<LastShares>,
    last_px: Option<LastPx>,
    last_spot_rate: Option<LastSpotRate>,
    last_forward_points: Option<LastForwardPoints>,
    last_mkt: Option<LastMkt>,
    trading_session_id: Option<TradingSessionID>,
    last_capacity: Option<LastCapacity>,
    leaves_qty: LeavesQty,
    cum_qty: CumQty,
    avg_px: AvgPx,
    day_order_qty: Option<DayOrderQty>,
    day_cum_qty: Option<DayCumQty>,
    day_avg_px: Option<DayAvgPx>,
    g_t_booking_inst: Option<GTBookingInst>,
    trade_date: Option<TradeDate>,
    transact_time: Option<TransactTime>,
    report_to_exch: Option<ReportToExch>,
    commission: Option<Commission>,
    comm_type: Option<CommType>,
    gross_trade_amt: Option<GrossTradeAmt>,
    settl_curr_amt: Option<SettlCurrAmt>,
    settl_currency: Option<SettlCurrency>,
    settl_curr_fx_rate: Option<SettlCurrFxRate>,
    settl_curr_fx_rate_calc: Option<SettlCurrFxRateCalc>,
    handl_inst: Option<HandlInst>,
    min_qty: Option<MinQty>,
    max_floor: Option<MaxFloor>,
    open_close: Option<OpenClose>,
    max_show: Option<MaxShow>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
    fut_sett_date2: Option<FutSettDate2>,
    order_qty2: Option<OrderQty2>,
    clearing_firm: Option<ClearingFirm>,
    clearing_account: Option<ClearingAccount>,
    multi_leg_reporting_type: Option<MultiLegReportingType>,
}
struct OrderCancelRejectMessage {
    order_id: OrderID,
    secondary_order_id: Option<SecondaryOrderID>,
    cl_ord_id: ClOrdID,
    orig_cl_ord_id: OrigClOrdID,
    ord_status: OrdStatus,
    client_id: Option<ClientID>,
    exec_broker: Option<ExecBroker>,
    list_id: Option<ListID>,
    account: Option<Account>,
    transact_time: Option<TransactTime>,
    cxl_rej_response_to: CxlRejResponseTo,
    cxl_rej_reason: Option<CxlRejReason>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct LogonMessage {
    encrypt_method: EncryptMethod,
    heart_bt_int: HeartBtInt,
    raw_data_length: Option<RawDataLength>,
    raw_data: Option<RawData>,
    reset_seq_num_flag: Option<ResetSeqNumFlag>,
    max_message_size: Option<MaxMessageSize>,
}
struct NewsMessage {
    orig_time: Option<OrigTime>,
    urgency: Option<Urgency>,
    headline: Headline,
    encoded_headline_len: Option<EncodedHeadlineLen>,
    encoded_headline: Option<EncodedHeadline>,
    u_rl_link: Option<URLLink>,
    raw_data_length: Option<RawDataLength>,
    raw_data: Option<RawData>,
}
struct EmailMessage {
    email_thread_id: EmailThreadID,
    email_type: EmailType,
    orig_time: Option<OrigTime>,
    subject: Subject,
    encoded_subject_len: Option<EncodedSubjectLen>,
    encoded_subject: Option<EncodedSubject>,
    order_id: Option<OrderID>,
    cl_ord_id: Option<ClOrdID>,
    raw_data_length: Option<RawDataLength>,
    raw_data: Option<RawData>,
}
struct NewOrderSingleMessage {
    cl_ord_id: ClOrdID,
    client_id: Option<ClientID>,
    exec_broker: Option<ExecBroker>,
    account: Option<Account>,
    settlmnt_typ: Option<SettlmntTyp>,
    fut_sett_date: Option<FutSettDate>,
    handl_inst: HandlInst,
    exec_inst: Option<ExecInst>,
    min_qty: Option<MinQty>,
    max_floor: Option<MaxFloor>,
    ex_destination: Option<ExDestination>,
    process_code: Option<ProcessCode>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    prev_close_px: Option<PrevClosePx>,
    side: Side,
    locate_reqd: Option<LocateReqd>,
    transact_time: TransactTime,
    order_qty: Option<OrderQty>,
    cash_order_qty: Option<CashOrderQty>,
    ord_type: OrdType,
    price: Option<Price>,
    stop_px: Option<StopPx>,
    currency: Option<Currency>,
    compliance_id: Option<ComplianceID>,
    solicited_flag: Option<SolicitedFlag>,
    i_o_iid: Option<IOIid>,
    quote_id: Option<QuoteID>,
    time_in_force: Option<TimeInForce>,
    effective_time: Option<EffectiveTime>,
    expire_date: Option<ExpireDate>,
    expire_time: Option<ExpireTime>,
    g_t_booking_inst: Option<GTBookingInst>,
    commission: Option<Commission>,
    comm_type: Option<CommType>,
    rule80_a: Option<Rule80A>,
    forex_req: Option<ForexReq>,
    settl_currency: Option<SettlCurrency>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
    fut_sett_date2: Option<FutSettDate2>,
    order_qty2: Option<OrderQty2>,
    open_close: Option<OpenClose>,
    covered_or_uncovered: Option<CoveredOrUncovered>,
    customer_or_firm: Option<CustomerOrFirm>,
    max_show: Option<MaxShow>,
    peg_difference: Option<PegDifference>,
    discretion_inst: Option<DiscretionInst>,
    discretion_offset: Option<DiscretionOffset>,
    clearing_firm: Option<ClearingFirm>,
    clearing_account: Option<ClearingAccount>,
}
struct NewOrderListMessage {
    list_id: ListID,
    bid_id: Option<BidID>,
    client_bid_id: Option<ClientBidID>,
    prog_rpt_reqs: Option<ProgRptReqs>,
    bid_type: BidType,
    prog_period_interval: Option<ProgPeriodInterval>,
    list_exec_inst_type: Option<ListExecInstType>,
    list_exec_inst: Option<ListExecInst>,
    encoded_list_exec_inst_len: Option<EncodedListExecInstLen>,
    encoded_list_exec_inst: Option<EncodedListExecInst>,
    tot_no_orders: TotNoOrders,
}
struct OrderCancelRequestMessage {
    orig_cl_ord_id: OrigClOrdID,
    order_id: Option<OrderID>,
    cl_ord_id: ClOrdID,
    list_id: Option<ListID>,
    account: Option<Account>,
    client_id: Option<ClientID>,
    exec_broker: Option<ExecBroker>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    side: Side,
    transact_time: TransactTime,
    order_qty: Option<OrderQty>,
    cash_order_qty: Option<CashOrderQty>,
    compliance_id: Option<ComplianceID>,
    solicited_flag: Option<SolicitedFlag>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct OrderCancelReplaceRequestMessage {
    order_id: Option<OrderID>,
    client_id: Option<ClientID>,
    exec_broker: Option<ExecBroker>,
    orig_cl_ord_id: OrigClOrdID,
    cl_ord_id: ClOrdID,
    list_id: Option<ListID>,
    account: Option<Account>,
    settlmnt_typ: Option<SettlmntTyp>,
    fut_sett_date: Option<FutSettDate>,
    handl_inst: HandlInst,
    exec_inst: Option<ExecInst>,
    min_qty: Option<MinQty>,
    max_floor: Option<MaxFloor>,
    ex_destination: Option<ExDestination>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    side: Side,
    transact_time: TransactTime,
    order_qty: Option<OrderQty>,
    cash_order_qty: Option<CashOrderQty>,
    ord_type: OrdType,
    price: Option<Price>,
    stop_px: Option<StopPx>,
    peg_difference: Option<PegDifference>,
    discretion_inst: Option<DiscretionInst>,
    discretion_offset: Option<DiscretionOffset>,
    compliance_id: Option<ComplianceID>,
    solicited_flag: Option<SolicitedFlag>,
    currency: Option<Currency>,
    time_in_force: Option<TimeInForce>,
    effective_time: Option<EffectiveTime>,
    expire_date: Option<ExpireDate>,
    expire_time: Option<ExpireTime>,
    g_t_booking_inst: Option<GTBookingInst>,
    commission: Option<Commission>,
    comm_type: Option<CommType>,
    rule80_a: Option<Rule80A>,
    forex_req: Option<ForexReq>,
    settl_currency: Option<SettlCurrency>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
    fut_sett_date2: Option<FutSettDate2>,
    order_qty2: Option<OrderQty2>,
    open_close: Option<OpenClose>,
    covered_or_uncovered: Option<CoveredOrUncovered>,
    customer_or_firm: Option<CustomerOrFirm>,
    max_show: Option<MaxShow>,
    locate_reqd: Option<LocateReqd>,
    clearing_firm: Option<ClearingFirm>,
    clearing_account: Option<ClearingAccount>,
}
struct OrderStatusRequestMessage {
    order_id: Option<OrderID>,
    cl_ord_id: ClOrdID,
    client_id: Option<ClientID>,
    account: Option<Account>,
    exec_broker: Option<ExecBroker>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    side: Side,
}
struct AllocationMessage {
    alloc_id: AllocID,
    alloc_trans_type: AllocTransType,
    ref_alloc_id: Option<RefAllocID>,
    alloc_link_id: Option<AllocLinkID>,
    alloc_link_type: Option<AllocLinkType>,
    side: Side,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    shares: Shares,
    last_mkt: Option<LastMkt>,
    trading_session_id: Option<TradingSessionID>,
    avg_px: AvgPx,
    currency: Option<Currency>,
    avg_prx_precision: Option<AvgPrxPrecision>,
    trade_date: TradeDate,
    transact_time: Option<TransactTime>,
    settlmnt_typ: Option<SettlmntTyp>,
    fut_sett_date: Option<FutSettDate>,
    gross_trade_amt: Option<GrossTradeAmt>,
    net_money: Option<NetMoney>,
    open_close: Option<OpenClose>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
    num_days_interest: Option<NumDaysInterest>,
    accrued_interest_rate: Option<AccruedInterestRate>,
}
struct ListCancelRequestMessage {
    list_id: ListID,
    transact_time: TransactTime,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct ListExecuteMessage {
    list_id: ListID,
    client_bid_id: Option<ClientBidID>,
    bid_id: Option<BidID>,
    transact_time: TransactTime,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct ListStatusRequestMessage {
    list_id: ListID,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct ListStatusMessage {
    list_id: ListID,
    list_status_type: ListStatusType,
    no_rpts: NoRpts,
    list_order_status: ListOrderStatus,
    rpt_seq: RptSeq,
    list_status_text: Option<ListStatusText>,
    encoded_list_status_text_len: Option<EncodedListStatusTextLen>,
    encoded_list_status_text: Option<EncodedListStatusText>,
    transact_time: Option<TransactTime>,
    tot_no_orders: TotNoOrders,
}
struct AllocationACKMessage {
    client_id: Option<ClientID>,
    exec_broker: Option<ExecBroker>,
    alloc_id: AllocID,
    trade_date: TradeDate,
    transact_time: Option<TransactTime>,
    alloc_status: AllocStatus,
    alloc_rej_code: Option<AllocRejCode>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct DontKnowTradeMessage {
    order_id: OrderID,
    exec_id: ExecID,
    d_k_reason: DKReason,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    side: Side,
    order_qty: Option<OrderQty>,
    cash_order_qty: Option<CashOrderQty>,
    last_shares: Option<LastShares>,
    last_px: Option<LastPx>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct QuoteRequestMessage {
    quote_req_id: QuoteReqID,
}
struct QuoteMessage {
    quote_req_id: Option<QuoteReqID>,
    quote_id: QuoteID,
    quote_response_level: Option<QuoteResponseLevel>,
    trading_session_id: Option<TradingSessionID>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    bid_px: Option<BidPx>,
    offer_px: Option<OfferPx>,
    bid_size: Option<BidSize>,
    offer_size: Option<OfferSize>,
    valid_until_time: Option<ValidUntilTime>,
    bid_spot_rate: Option<BidSpotRate>,
    offer_spot_rate: Option<OfferSpotRate>,
    bid_forward_points: Option<BidForwardPoints>,
    offer_forward_points: Option<OfferForwardPoints>,
    transact_time: Option<TransactTime>,
    fut_sett_date: Option<FutSettDate>,
    ord_type: Option<OrdType>,
    fut_sett_date2: Option<FutSettDate2>,
    order_qty2: Option<OrderQty2>,
    currency: Option<Currency>,
}
struct SettlementInstructionsMessage {
    settl_inst_id: SettlInstID,
    settl_inst_trans_type: SettlInstTransType,
    settl_inst_ref_id: SettlInstRefID,
    settl_inst_mode: SettlInstMode,
    settl_inst_source: SettlInstSource,
    alloc_account: AllocAccount,
    settl_location: Option<SettlLocation>,
    trade_date: Option<TradeDate>,
    alloc_id: Option<AllocID>,
    last_mkt: Option<LastMkt>,
    trading_session_id: Option<TradingSessionID>,
    side: Option<Side>,
    security_type: Option<SecurityType>,
    effective_time: Option<EffectiveTime>,
    transact_time: TransactTime,
    client_id: Option<ClientID>,
    exec_broker: Option<ExecBroker>,
    stand_inst_db_type: Option<StandInstDbType>,
    stand_inst_db_name: Option<StandInstDbName>,
    stand_inst_db_id: Option<StandInstDbID>,
    settl_delivery_type: Option<SettlDeliveryType>,
    settl_depository_code: Option<SettlDepositoryCode>,
    settl_brkr_code: Option<SettlBrkrCode>,
    settl_inst_code: Option<SettlInstCode>,
    security_settl_agent_name: Option<SecuritySettlAgentName>,
    security_settl_agent_code: Option<SecuritySettlAgentCode>,
    security_settl_agent_acct_num: Option<SecuritySettlAgentAcctNum>,
    security_settl_agent_acct_name: Option<SecuritySettlAgentAcctName>,
    security_settl_agent_contact_name: Option<SecuritySettlAgentContactName>,
    security_settl_agent_contact_phone: Option<SecuritySettlAgentContactPhone>,
    cash_settl_agent_name: Option<CashSettlAgentName>,
    cash_settl_agent_code: Option<CashSettlAgentCode>,
    cash_settl_agent_acct_num: Option<CashSettlAgentAcctNum>,
    cash_settl_agent_acct_name: Option<CashSettlAgentAcctName>,
    cash_settl_agent_contact_name: Option<CashSettlAgentContactName>,
    cash_settl_agent_contact_phone: Option<CashSettlAgentContactPhone>,
}
struct MarketDataRequestMessage {
    m_d_req_id: MDReqID,
    subscription_request_type: SubscriptionRequestType,
    market_depth: MarketDepth,
    m_d_update_type: Option<MDUpdateType>,
    aggregated_book: Option<AggregatedBook>,
}
struct MarketDataSnapshotFullRefreshMessage {
    m_d_req_id: Option<MDReqID>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    financial_status: Option<FinancialStatus>,
    corporate_action: Option<CorporateAction>,
    total_volume_traded: Option<TotalVolumeTraded>,
}
struct MarketDataIncrementalRefreshMessage {
    m_d_req_id: Option<MDReqID>,
}
struct MarketDataRequestRejectMessage {
    m_d_req_id: MDReqID,
    m_d_req_rej_reason: Option<MDReqRejReason>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct QuoteCancelMessage {
    quote_req_id: Option<QuoteReqID>,
    quote_id: QuoteID,
    quote_cancel_type: QuoteCancelType,
    quote_response_level: Option<QuoteResponseLevel>,
    trading_session_id: Option<TradingSessionID>,
}
struct QuoteStatusRequestMessage {
    quote_id: Option<QuoteID>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    side: Option<Side>,
    trading_session_id: Option<TradingSessionID>,
}
struct QuoteAcknowledgementMessage {
    quote_req_id: Option<QuoteReqID>,
    quote_id: Option<QuoteID>,
    quote_ack_status: QuoteAckStatus,
    quote_reject_reason: Option<QuoteRejectReason>,
    quote_response_level: Option<QuoteResponseLevel>,
    trading_session_id: Option<TradingSessionID>,
    text: Option<Text>,
}
struct SecurityDefinitionRequestMessage {
    security_req_id: SecurityReqID,
    security_request_type: SecurityRequestType,
    symbol: Option<Symbol>,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    currency: Option<Currency>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
    trading_session_id: Option<TradingSessionID>,
}
struct SecurityDefinitionMessage {
    security_req_id: SecurityReqID,
    security_response_id: SecurityResponseID,
    security_response_type: Option<SecurityResponseType>,
    total_num_securities: TotalNumSecurities,
    symbol: Option<Symbol>,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    currency: Option<Currency>,
    trading_session_id: Option<TradingSessionID>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct SecurityStatusRequestMessage {
    security_status_req_id: SecurityStatusReqID,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    currency: Option<Currency>,
    subscription_request_type: SubscriptionRequestType,
    trading_session_id: Option<TradingSessionID>,
}
struct SecurityStatusMessage {
    security_status_req_id: Option<SecurityStatusReqID>,
    symbol: Symbol,
    symbol_sfx: Option<SymbolSfx>,
    security_id: Option<SecurityID>,
    i_d_source: Option<IDSource>,
    security_type: Option<SecurityType>,
    maturity_month_year: Option<MaturityMonthYear>,
    maturity_day: Option<MaturityDay>,
    put_or_call: Option<PutOrCall>,
    strike_price: Option<StrikePrice>,
    opt_attribute: Option<OptAttribute>,
    contract_multiplier: Option<ContractMultiplier>,
    coupon_rate: Option<CouponRate>,
    security_exchange: Option<SecurityExchange>,
    issuer: Option<Issuer>,
    encoded_issuer_len: Option<EncodedIssuerLen>,
    encoded_issuer: Option<EncodedIssuer>,
    security_desc: Option<SecurityDesc>,
    encoded_security_desc_len: Option<EncodedSecurityDescLen>,
    encoded_security_desc: Option<EncodedSecurityDesc>,
    currency: Option<Currency>,
    trading_session_id: Option<TradingSessionID>,
    unsolicited_indicator: Option<UnsolicitedIndicator>,
    security_trading_status: Option<SecurityTradingStatus>,
    financial_status: Option<FinancialStatus>,
    corporate_action: Option<CorporateAction>,
    halt_reason_char: Option<HaltReasonChar>,
    in_view_of_common: Option<InViewOfCommon>,
    due_to_related: Option<DueToRelated>,
    buy_volume: Option<BuyVolume>,
    sell_volume: Option<SellVolume>,
    high_px: Option<HighPx>,
    low_px: Option<LowPx>,
    last_px: Option<LastPx>,
    transact_time: Option<TransactTime>,
    adjustment: Option<Adjustment>,
}
struct TradingSessionStatusRequestMessage {
    trad_ses_req_id: TradSesReqID,
    trading_session_id: Option<TradingSessionID>,
    trad_ses_method: Option<TradSesMethod>,
    trad_ses_mode: Option<TradSesMode>,
    subscription_request_type: SubscriptionRequestType,
}
struct TradingSessionStatusMessage {
    trad_ses_req_id: Option<TradSesReqID>,
    trading_session_id: TradingSessionID,
    trad_ses_method: Option<TradSesMethod>,
    trad_ses_mode: Option<TradSesMode>,
    unsolicited_indicator: Option<UnsolicitedIndicator>,
    trad_ses_status: TradSesStatus,
    trad_ses_start_time: Option<TradSesStartTime>,
    trad_ses_open_time: Option<TradSesOpenTime>,
    trad_ses_pre_close_time: Option<TradSesPreCloseTime>,
    trad_ses_close_time: Option<TradSesCloseTime>,
    trad_ses_end_time: Option<TradSesEndTime>,
    total_volume_traded: Option<TotalVolumeTraded>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct MassQuoteMessage {
    quote_req_id: Option<QuoteReqID>,
    quote_id: QuoteID,
    quote_response_level: Option<QuoteResponseLevel>,
    def_bid_size: Option<DefBidSize>,
    def_offer_size: Option<DefOfferSize>,
}
struct BusinessMessageRejectMessage {
    ref_seq_num: Option<RefSeqNum>,
    ref_msg_type: RefMsgType,
    business_reject_ref_id: Option<BusinessRejectRefID>,
    business_reject_reason: BusinessRejectReason,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct BidRequestMessage {
    bid_id: Option<BidID>,
    client_bid_id: ClientBidID,
    bid_request_trans_type: BidRequestTransType,
    list_name: Option<ListName>,
    total_num_securities: TotalNumSecurities,
    bid_type: BidType,
    num_tickets: Option<NumTickets>,
    currency: Option<Currency>,
    side_value1: Option<SideValue1>,
    side_value2: Option<SideValue2>,
    liquidity_ind_type: Option<LiquidityIndType>,
    wt_average_liquidity: Option<WtAverageLiquidity>,
    exchange_for_physical: Option<ExchangeForPhysical>,
    out_main_cntry_u_index: Option<OutMainCntryUIndex>,
    cross_percent: Option<CrossPercent>,
    prog_rpt_reqs: Option<ProgRptReqs>,
    prog_period_interval: Option<ProgPeriodInterval>,
    inc_tax_ind: Option<IncTaxInd>,
    forex_req: Option<ForexReq>,
    num_bidders: Option<NumBidders>,
    trade_date: Option<TradeDate>,
    trade_type: TradeType,
    basis_px_type: BasisPxType,
    strike_time: Option<StrikeTime>,
    text: Option<Text>,
    encoded_text_len: Option<EncodedTextLen>,
    encoded_text: Option<EncodedText>,
}
struct BidResponseMessage {
    bid_id: Option<BidID>,
    client_bid_id: Option<ClientBidID>,
}
struct ListStrikePriceMessage {
    list_id: ListID,
    tot_no_strikes: TotNoStrikes,
}
