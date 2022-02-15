// Copyright 2021 Datafuse Labs.
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

use common_datavalues2::prelude::*;
use common_exception::Result;
use common_functions::scalars::*;

use crate::scalars::scalar_function2_test::test_scalar_functions2;
use crate::scalars::scalar_function2_test::ScalarFunction2Test;

#[test]
fn test_exp_function() -> Result<()> {
    let tests = vec![
        ScalarFunction2Test {
            name: "exp-with-literal",
            columns: vec![Series::from_data(vec![2])],
            expect: Series::from_data(vec![7.38905609893065_f64]),
            error: "",
        },
        ScalarFunction2Test {
            name: "exp-with-series",
            columns: vec![Series::from_data(vec![2, -2, 0])],
            expect: Series::from_data(vec![7.38905609893065_f64, 0.1353352832366127, 1_f64]),
            error: "",
        },
    ];

    test_scalar_functions2(ExpFunction::try_create("exp")?, &tests)
}
