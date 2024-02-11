//! A brazilian payment gateway.
//!
//! <https://docs.pagar.me/>
//!
//! ```rust,no_run
//! # async fn fun() -> wtx_apis::Result<()> {
//! use wtx::client_api_framework::{dnsn::SerdeJson, network::HttpParams};
//! use wtx_apis::payment_gateway::pagar_me::{PagarMe, PkgsAux};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(PagarMe::new("KEY")?, SerdeJson, HttpParams::from_uri("URL"));
//! let _ = pkgs_aux.recipient_balance().params("0").build();
//! # Ok(()) }
//! ```

wtx::create_packages_aux_wrapper!();

mod address;
//#[cfg(all(test, feature = "_integration-tests"))]
//mod integration_tests;
mod pagar_me_error;
mod pagar_me_response;
mod phone;
mod pkg;

use arrayvec::ArrayString;
use base64::{engine::general_purpose::STANDARD, Engine};
use core::{
  fmt::{Debug, Formatter},
  str,
  time::Duration,
};
pub use pagar_me_response::PagarMeResponse;
pub use phone::*;
pub use pkg::*;
use wtx::{
  client_api_framework::{
    misc::{RequestLimit, RequestThrottling},
    Api,
  },
  misc::from_utf8_basic,
};

const MAX_API_KEY_LEN: usize = 64;

#[doc = _generic_api_doc!()]
#[wtx_macros::api_types(pkgs_aux(PkgsAux), transport(http))]
pub struct PagarMe {
  api_key: ArrayString<MAX_API_KEY_LEN>,
  rt_150: RequestThrottling,
}

impl PagarMe {
  #[inline]
  pub fn new(api_key: &str) -> crate::Result<Self> {
    const _1_MIN: Duration = Duration::from_secs(30);
    Ok(Self {
      api_key: {
        let fun = || {
          let total_len = api_key.len().wrapping_add(1);
          if total_len > MAX_API_KEY_LEN {
            return None;
          }
          let mut total_buffer = [0; MAX_API_KEY_LEN];
          let total = total_buffer.get_mut(..total_len)?;
          let [initial @ .., last] = total else {
            return None;
          };
          initial.copy_from_slice(api_key.as_bytes());
          *last = b':';
          let mut base64_buffer = [0; MAX_API_KEY_LEN];
          let n = STANDARD.encode_slice(total, &mut base64_buffer).ok()?;
          from_utf8_basic(base64_buffer.get(..n)?).ok()?.try_into().ok()
        };
        fun().unwrap_or_default()
      },
      rt_150: RequestThrottling::from_rl(RequestLimit::new(150, _1_MIN)?)?,
    })
  }
}

impl Api for PagarMe {
  type Error = crate::Error;
}

impl Debug for PagarMe {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("PagarMe").finish()
  }
}
