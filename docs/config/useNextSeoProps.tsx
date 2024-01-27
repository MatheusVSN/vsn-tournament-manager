import { NextSeoProps } from "next-seo"
import { useRouter } from "next/router"
import { DESCRIPTION } from "./constants"

export function useNextSeoProps() {
  const { route } = useRouter()
  const result: NextSeoProps = {
    description: DESCRIPTION,
  }
  if (route !== "/") {
    result.titleTemplate = "%s | VSN-Tournament Manager"
  } else {
    result.titleTemplate = "%s"
  }

  return result
}
