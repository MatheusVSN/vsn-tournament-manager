import { DocsThemeConfig } from "nextra-theme-docs"
import { REPO_NAME } from "./config/constants"
import { useNextSeoProps } from "./config/useNextSeoProps"

const link = `https://github.com/MatheusVSN/${REPO_NAME}`

const config: DocsThemeConfig = {
  logo: <span>VSN-Tournament Manager</span>,
  project: {
    link,
  },
  docsRepositoryBase: `${link}/tree/main/docs`,
  footer: {
    text: "VSN-Tournament Manager",
  },
  useNextSeoProps,
  feedback: {
    content: <>Question? Give me a feedback</>,
  },
}

export default config
