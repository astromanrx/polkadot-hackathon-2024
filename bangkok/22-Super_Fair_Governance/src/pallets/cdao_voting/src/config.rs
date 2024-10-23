//pub mod weights;
use frame_support::pallet_macros::pallet_section;
// Re-export pallet items so that they can be accessed from the crate namespace.

/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod config {
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;

        /// A random value generator.
        // type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

        #[pallet::constant]
        type MaxBidEntries: Get<u32>;
    }
}