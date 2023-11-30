import { HomePageTitle } from '@/components/home/HomePageTitle'
import { BaseLayout } from '@/components/layout/BaseLayout'
import { CenterBody } from '@/components/layout/CenterBody'
import { Header } from '@/components/layout/Header'
import { HotToastConfig } from '@/components/layout/HotToastConfig'
import { env } from '@/config/environment'
import { getDeployments } from '@/deployments/deployments'
import GlobalStyles from '@/styles/GlobalStyles'
import { ChakraProvider, DarkMode } from '@chakra-ui/react'
import { cache } from '@emotion/css'
import { CacheProvider } from '@emotion/react'
import { UseInkathonProvider } from '@scio-labs/use-inkathon'
import { DefaultSeo } from 'next-seo'
import type { AppProps } from 'next/app'
import { Inconsolata } from 'next/font/google'
import Head from 'next/head'

// Google Font(s) via `next/font`
const inconsolata = Inconsolata({ subsets: ['latin'] })

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <DefaultSeo
        dangerouslySetAllPagesToNoFollow={!env.isProduction}
        dangerouslySetAllPagesToNoIndex={!env.isProduction}
        defaultTitle="kudos ink!"
        titleTemplate="%s | kudos ink!"
        description="A minimalist kudos collector service for rewarded open source contributions."
        openGraph={{
          type: 'website',
          locale: 'en',
          url: env.url,
          site_name: 'kudos ink!',
          images: [
            {
              url: `${env.url}/images/cover.jpg`,
              width: 1200,
              height: 675,
            },
          ],
        }}
        twitter={{
          handle: '', // DO
        }}
      />

      <Head>
        <meta name="viewport" content="initial-scale=1.0, width=device-width" />

        {/* Set Font Variables */}
        <style>{`
          :root {
            --font-inconsolata: ${inconsolata.style.fontFamily}, 'Inconsolata';
          }
        `}</style>
      </Head>

      <UseInkathonProvider
        appName="kudos"
        connectOnInit={true}
        defaultChain={env.defaultChain}
        deployments={getDeployments()}
      >
        <CacheProvider value={cache}>
          <ChakraProvider>
            <DarkMode>
              <GlobalStyles />
              <Header />

              <BaseLayout>
                <CenterBody tw="mt-32 mb-10 px-5 lg:mt-0">
                  {/* Title */}
                  <HomePageTitle />

                  <Component {...pageProps} />
                </CenterBody>
              </BaseLayout>

              <HotToastConfig />
            </DarkMode>
          </ChakraProvider>
        </CacheProvider>
      </UseInkathonProvider>
    </>
  )
}

export default MyApp
