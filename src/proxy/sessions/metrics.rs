/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use once_cell::sync::Lazy;
use prometheus::{Histogram, IntCounter, IntGauge, IntGaugeVec, Opts};

use crate::metrics::{histogram_opts, register};

const SUBSYSTEM: &str = "session";
const ASN_NUMBER_LABEL: &str = "asn";
const IP_PREFIX_LABEL: &str = "ip_prefix";

pub(crate) fn active_sessions(asn_number: u16, ip_prefix: &str) -> IntGauge {
    static ACTIVE_SESSIONS: Lazy<IntGaugeVec> = Lazy::new(|| {
        prometheus::register_int_gauge_vec_with_registry! {
            Opts::new("active", "number of sessions currently active").subsystem(SUBSYSTEM),
            &[ASN_NUMBER_LABEL, IP_PREFIX_LABEL],
            crate::metrics::registry(),
        }
        .unwrap()
    });

    ACTIVE_SESSIONS.with_label_values(&[&asn_number.to_string(), ip_prefix])
}

pub(crate) fn total_sessions() -> &'static IntCounter {
    static TOTAL_SESSIONS: Lazy<IntCounter> = Lazy::new(|| {
        register(
            IntCounter::with_opts(
                Opts::new("total", "total number of established sessions").subsystem(SUBSYSTEM),
            )
            .unwrap(),
        )
    });

    &TOTAL_SESSIONS
}

pub(crate) fn duration_secs() -> &'static Histogram {
    static DURATION_SECS: Lazy<Histogram> = Lazy::new(|| {
        register(
            Histogram::with_opts(histogram_opts(
                "duration_secs",
                SUBSYSTEM,
                "duration of sessions",
                vec![
                    1f64, 5f64, 10f64, 25f64, 60f64, 300f64, 900f64, 1800f64, 3600f64,
                ],
            ))
            .unwrap(),
        )
    });

    &DURATION_SECS
}
