import { useContext } from "react";
import { LocaleContext, type LocaleContextValue } from "./locale-context";

export function useLocale(): LocaleContextValue {
  return useContext(LocaleContext);
}
