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

use std::sync::Arc;

use common_datavalues::prelude::*;
use common_exception::Result;
use common_io::prelude::FormatSettings;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn test_serializers() -> Result<()> {
    struct Test {
        name: &'static str,
        data_type: DataTypeImpl,
        value: DataValue,
        column: ColumnRef,
        val_str: &'static str,
        col_str: Vec<String>,
    }

    let tests = vec![
        Test {
            name: "boolean",
            data_type: BooleanType::new_impl(),
            value: DataValue::Boolean(true),
            column: Series::from_data(vec![true, false, true]),
            val_str: "1",
            col_str: vec!["1".to_owned(), "0".to_owned(), "1".to_owned()],
        },
        Test {
            name: "int8",
            data_type: Int8Type::new_impl(),
            value: DataValue::Int64(1),
            column: Series::from_data(vec![1i8, 2i8, 1]),
            val_str: "1",
            col_str: vec!["1".to_owned(), "2".to_owned(), "1".to_owned()],
        },
        Test {
            name: "datetime32",
            data_type: TimestampType::new_impl(0),
            value: DataValue::UInt64(1630320462000000),
            column: Series::from_data(vec![1630320462000000i64, 1637117572000000i64, 1000000]),
            val_str: "2021-08-30 10:47:42",
            col_str: vec![
                "2021-08-30 10:47:42".to_owned(),
                "2021-11-17 02:52:52".to_owned(),
                "1970-01-01 00:00:01".to_owned(),
            ],
        },
        Test {
            name: "date32",
            data_type: DateType::new_impl(),
            value: DataValue::Int64(18869),
            column: Series::from_data(vec![18869i32, 18948i32, 1]),
            val_str: "2021-08-30",
            col_str: vec![
                "2021-08-30".to_owned(),
                "2021-11-17".to_owned(),
                "1970-01-02".to_owned(),
            ],
        },
        Test {
            name: "string",
            data_type: StringType::new_impl(),
            value: DataValue::String("hello".as_bytes().to_vec()),
            column: Series::from_data(vec!["hello", "world", "NULL"]),
            val_str: "hello",
            col_str: vec!["hello".to_owned(), "world".to_owned(), "NULL".to_owned()],
        },
        Test {
            name: "array",
            data_type: DataTypeImpl::Array(ArrayType::create(StringType::new_impl())),
            value: DataValue::Array(vec![
                DataValue::String("data".as_bytes().to_vec()),
                DataValue::String("bend".as_bytes().to_vec()),
            ]),
            column: Arc::new(ArrayColumn::from_data(
                DataTypeImpl::Array(ArrayType::create(StringType::new_impl())),
                vec![0, 1, 3, 6].into(),
                Series::from_data(vec!["test", "data", "bend", "hello", "world", "NULL"]),
            )),
            val_str: "['data', 'bend']",
            col_str: vec![
                "['test']".to_owned(),
                "['data', 'bend']".to_owned(),
                "['hello', 'world', 'NULL']".to_owned(),
            ],
        },
        Test {
            name: "struct",
            data_type: DataTypeImpl::Struct(StructType::create(
                vec!["date".to_owned(), "integer".to_owned()],
                vec![DateType::new_impl(), Int8Type::new_impl()],
            )),
            value: DataValue::Struct(vec![DataValue::Int64(18869), DataValue::Int64(1)]),
            column: Arc::new(StructColumn::from_data(
                vec![
                    Series::from_data(vec![18869i32, 18948i32, 1]),
                    Series::from_data(vec![1i8, 2i8, 3]),
                ],
                DataTypeImpl::Struct(StructType::create(
                    vec!["date".to_owned(), "integer".to_owned()],
                    vec![DateType::new_impl(), Int8Type::new_impl()],
                )),
            )),
            val_str: "('2021-08-30', 1)",
            col_str: vec![
                "('2021-08-30', 1)".to_owned(),
                "('2021-11-17', 2)".to_owned(),
                "('1970-01-02', 3)".to_owned(),
            ],
        },
        Test {
            name: "variant",
            data_type: VariantType::new_impl(),
            value: DataValue::Variant(VariantValue::from(json!(true))),
            column: Arc::new(VariantColumn::new_from_vec(vec![
                VariantValue::from(json!(null)),
                VariantValue::from(json!(true)),
                VariantValue::from(json!(false)),
                VariantValue::from(json!(123)),
                VariantValue::from(json!(12.34)),
            ])),
            val_str: "true",
            col_str: vec![
                "null".to_owned(),
                "true".to_owned(),
                "false".to_owned(),
                "123".to_owned(),
                "12.34".to_owned(),
            ],
        },
    ];

    let format = FormatSettings::default();
    for test in tests {
        let serializer = test.data_type.create_serializer();
        let val_res = serializer.serialize_value(&test.value, &format)?;
        assert_eq!(&val_res, test.val_str, "case: {:#?}", test.name);

        let col_res = serializer.serialize_column(&test.column, &format)?;
        assert_eq!(col_res, test.col_str, "case: {:#?}", test.name);
    }

    {
        let data_type = StructType::create(
            vec![
                "item_1".to_owned(),
                "item_2".to_owned(),
                "item_3".to_owned(),
                "item_4".to_owned(),
            ],
            vec![
                Float64Type::new_impl(),
                StringType::new_impl(),
                BooleanType::new_impl(),
                DateType::new_impl(),
            ],
        );
        let serializer = data_type.create_serializer();
        let value = DataValue::Struct(vec![
            DataValue::Float64(1.2),
            DataValue::String("hello".as_bytes().to_vec()),
            DataValue::Boolean(true),
            DataValue::UInt64(18869),
        ]);
        let result = serializer.serialize_value(&value, &format)?;
        let expect = "(1.2, 'hello', 1, '2021-08-30')";
        assert_eq!(&result, expect);
    }

    Ok(())
}

#[test]
fn test_convert_arrow() {
    let t = TimestampType::new_impl(0);
    let arrow_y = t.to_arrow_field("x");
    let new_t = from_arrow_field(&arrow_y);

    assert_eq!(new_t.name(), t.name())
}

#[test]
fn test_enum_dispatch() -> Result<()> {
    let c = StringSerializer {};
    let d: TypeSerializerImpl = c.into();
    let _: StringSerializer = d.try_into()?;
    Ok(())
}
