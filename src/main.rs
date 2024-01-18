slint::include_modules!();

const TAXPER: f64 = 0.27;
const OWNPER: f64 = 0.55;
const PROFITPER: f64 = 0.06;
const OPEXPER: f64 = 0.12;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui: AppWindow = ui_handle.unwrap();

        let num: f64 = string.trim().parse().unwrap();
        let tax: f64 = num * TAXPER;
        let owner: f64 = num * OWNPER;
        let profit: f64 = num * PROFITPER;
        let oprexp: f64 = num * OPEXPER;

        let res: String = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperation: {:.2}", tax, owner, profit, oprexp);
        ui.set_result(res.into());
    });

    ui.run()
}
