use quanta_engine::{MarketParameters, validate_market_parameters, process_market_parameters};

#[test]
fn test_validate_market_parameters() {
    let valid_params = MarketParameters {
        risk_free_rate: 0.02,
        market_volatility: 0.15,
        expected_return: 0.08,
    };
    assert!(validate_market_parameters(&valid_params).is_ok());

    let invalid_params = MarketParameters {
        risk_free_rate: -0.01,
        market_volatility: 0.15,
        expected_return: 0.08,
    };
    assert!(validate_market_parameters(&invalid_params).is_err());
}

#[test]
fn test_process_market_parameters() {
    let valid_params = r#"
    {
        "risk_free_rate": 0.02,
        "market_volatility": 0.15,
        "expected_return": 0.08
    }
    "#;
    assert!(process_market_parameters(valid_params.to_string()).is_ok());

    let invalid_params = r#"
    {
        "risk_free_rate": -0.01,
        "market_volatility": 0.15,
        "expected_return": 0.08
    }
    "#;
    assert!(process_market_parameters(invalid_params.to_string()).is_err());
}