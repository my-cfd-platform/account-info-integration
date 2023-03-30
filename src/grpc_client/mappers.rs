use crate::{
    report_grpc::{
        ReportsFlowsActivePositionGrpcModel, ReportsFlowsBidAsk, ReportsFlowsClosePositionReason,
        ReportsFlowsClosedPositionGrpcModel,
    },
    trading_info_integration_grpc::{
        ActiveOrderGrpcModel, ClosedOrderGrpcModel, PositionCloseReason, TradingInfoBidAskModel,
    },
};

impl Into<TradingInfoBidAskModel> for ReportsFlowsBidAsk {
    fn into(self) -> TradingInfoBidAskModel {
        TradingInfoBidAskModel {
            id: self.asset_pair,
            bid: self.bid,
            ask: self.ask,
            date_time: self.date_time_unix_timestamp_milis,
        }
    }
}

impl Into<PositionCloseReason> for ReportsFlowsClosePositionReason {
    fn into(self) -> PositionCloseReason {
        match self {
            ReportsFlowsClosePositionReason::ClientCommand => PositionCloseReason::ClientCommand,
            ReportsFlowsClosePositionReason::StopOut => PositionCloseReason::StopOut,
            ReportsFlowsClosePositionReason::TakeProfit => PositionCloseReason::TakeProfit,
            ReportsFlowsClosePositionReason::StopLoss => PositionCloseReason::StopLoss,
            ReportsFlowsClosePositionReason::ForceClose => PositionCloseReason::ForceClose,
        }
    }
}

impl Into<ActiveOrderGrpcModel> for ReportsFlowsActivePositionGrpcModel {
    fn into(self) -> ActiveOrderGrpcModel {
        ActiveOrderGrpcModel {
            id: self.id,
            trader_id: self.trader_id,
            account_id: self.account_id,
            instrument_id: self.asset_pair,
            invest_amount: self.invest_amount,
            leverage: self.leverage,
            create_date: self.create_date_unix_timestamp_milis,
            position_side: self.side,
            tp_in_position_pl: self.tp_in_profit,
            tp_in_asset_price: self.tp_in_asset_price,
            sl_in_position_pl: self.sl_in_profit,
            sl_in_asset_price: self.sl_in_asset_price,
            last_update_date: self.last_update_date,
            create_process_id: self.create_process_id,
            volume: self.leverage * self.invest_amount,
            profit: 0.0,
            so_percent: self.stop_out_percent,
            desire_price: None,
            open_price: self.open_price,
            open_bid_ask: Some(self.open_bid_ask.unwrap().into()),
            open_date: self.open_date,
        }
    }
}

impl Into<ClosedOrderGrpcModel> for ReportsFlowsClosedPositionGrpcModel {
    fn into(self) -> ClosedOrderGrpcModel {
        let status: PositionCloseReason =
            ReportsFlowsClosePositionReason::from_i32(self.close_reason)
                .unwrap()
                .into();
        ClosedOrderGrpcModel {
            id: self.id,
            trader_id: self.trader_id,
            account_id: self.account_id,
            instrument_id: self.asset_pair,
            invest_amount: self.invest_amount,
            leverage: self.leverage,
            create_date: self.create_date_unix_timestamp_milis,
            position_side: self.side,
            tp_in_position_pl: self.tp_in_profit,
            tp_in_asset_price: self.tp_in_asset_price,
            sl_in_position_pl: self.sl_in_profit,
            sl_in_asset_price: self.sl_in_asset_price,
            last_update_date: self.last_update_date,
            create_process_id: self.create_process_id,
            volume: self.leverage * self.invest_amount,
            profit: self.profit,
            so_percent: self.stop_out_percent,
            desire_price: None,
            open_price: self.open_price,
            open_bid_ask: Some(self.open_bid_ask.unwrap().into()),
            open_date: self.open_date,
            close_bid_ask: Some(self.close_bid_ask.unwrap().into()),
            close_price: self.close_price,
            close_date: self.close_date,
            close_reason: status as i32,
        }
    }
}
