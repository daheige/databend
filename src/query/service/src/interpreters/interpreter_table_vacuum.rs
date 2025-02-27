// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_exception::ErrorCode;
use common_exception::Result;
use common_license::license_manager::get_license_manager;
use common_sql::plans::VacuumTablePlan;
use common_storages_fuse::FuseTable;
use vacuum_handler::get_vacuum_handler;

use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;
use crate::sessions::TableContext;

#[allow(dead_code)]
pub struct VacuumTableInterpreter {
    ctx: Arc<QueryContext>,
    plan: VacuumTablePlan,
}

impl VacuumTableInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: VacuumTablePlan) -> Result<Self> {
        Ok(VacuumTableInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for VacuumTableInterpreter {
    fn name(&self) -> &str {
        "VacuumTableInterpreter"
    }

    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let license_manager = get_license_manager();
        if !license_manager.manager.is_active() {
            return Err(ErrorCode::LicenceDenied(
                "Need Commercial License".to_string(),
            ));
        }

        let catalog_name = self.plan.catalog.clone();
        let db_name = self.plan.database.clone();
        let tbl_name = self.plan.table.clone();
        let ctx = self.ctx.clone();
        let table = self
            .ctx
            .get_table(&catalog_name, &db_name, &tbl_name)
            .await?;
        let retention_time = chrono::Utc::now()
            - chrono::Duration::hours(ctx.get_settings().get_retention_period()? as i64);
        let build_res = PipelineBuildResult::create();
        let ctx = self.ctx.clone();

        let fuse_table = FuseTable::try_from_table(table.as_ref())?;
        let handler = get_vacuum_handler();
        handler.do_vacuum(fuse_table, ctx, retention_time).await?;

        Ok(build_res)
    }
}
