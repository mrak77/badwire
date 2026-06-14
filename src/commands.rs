use crate::tc;
use crate::state::AppState;
use crate::parameter_entries::ParsedParams;
use crate::error::AppError;

pub fn start_netem(
    state: &AppState,
    iface: &str,
    params: &ParsedParams,
) -> Result<(), AppError> {
    let args = tc::build_netem_args(
        &params.delay,
        params.jitter.as_deref(),
        params.loss.as_deref(),
        params.loss_corr.as_deref(),
        params.reorder.as_deref(),
        params.reorder_corr.as_deref(),
        params.corrupt.as_deref(),
        params.corrupt_corr.as_deref(),
        params.duplicate.as_deref(),
        params.duplicate_corr.as_deref(),
    )?;

    let mut full_args: Vec<String> = vec![
        "qdisc".to_string(),
        "replace".to_string(),
        "dev".to_string(),
        iface.to_string(),
        "root".to_string(),
        "netem".to_string(),
    ];
    full_args.extend(args);

    let slice: Vec<&str> = full_args.iter().map(|s| s.as_str()).collect();
    tc::run_tc(&slice)?;

    // Обновляем состояние через безопасные методы
    state.set_active(true);
    state.set_selected_iface(iface);
    state.set_current_config(&params.describe());
    Ok(())
}

pub fn stop_netem(state: &AppState, iface: &str) -> Result<(), AppError> {
    // Полный сброс: qdisc replace dev <iface> root netem без аргументов
    let args = ["qdisc", "replace", "dev", iface, "root", "netem"];
    tc::run_tc(&args)?;

    state.set_active(false);
    state.set_current_config("No active configuration");
    Ok(())
}
