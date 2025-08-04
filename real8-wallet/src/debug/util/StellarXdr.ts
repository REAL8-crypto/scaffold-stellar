import init, {
  decode,
  encode,
  guess,
  decode_stream,
} from "@stellar/stellar-xdr-json";
import wasm from "@stellar/stellar-xdr-json/stellar_xdr_json_bg.wasm?url";

// A wrapper for the Stellar XDR JSON
declare global {
  interface Window {
    __STELLAR_XDR_INIT__?: boolean;
  }
}

const initialize = async () => {
  if (!window.__STELLAR_XDR_INIT__) {
    await init(wasm);
    window.__STELLAR_XDR_INIT__ = true;
  }
};

export { initialize, decode, decode_stream, encode, guess };
