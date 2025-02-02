use rquickjs::{
	function::Func,
	module::{Declarations, Exports, ModuleDef},
	Ctx,
	Function,
	Result,
};

use super::console::format_plain;
use crate::{module_builder::ModuleInfo, modules::module::export_default};

pub struct UtilModule;

impl ModuleDef for UtilModule {
	fn declare(declare:&Declarations) -> Result<()> {
		declare.declare(stringify!(TextDecoder))?;

		declare.declare(stringify!(TextEncoder))?;

		declare.declare(stringify!(format))?;

		declare.declare("default")?;

		Ok(())
	}

	fn evaluate<'js>(ctx:&Ctx<'js>, exports:&Exports<'js>) -> Result<()> {
		export_default(ctx, exports, |default| {
			let globals = ctx.globals();

			let encoder:Function = globals.get(stringify!(TextEncoder))?;

			let decoder:Function = globals.get(stringify!(TextDecoder))?;

			default.set(stringify!(TextEncoder), encoder)?;

			default.set(stringify!(TextDecoder), decoder)?;

			default.set("format", Func::from(format_plain))?;

			Ok(())
		})
	}
}

impl From<UtilModule> for ModuleInfo<UtilModule> {
	fn from(val:UtilModule) -> Self { ModuleInfo { name:"util", module:val } }
}
