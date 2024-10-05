import '../styles/globals.css';
import type { AppProps } from 'next/app';
import { ToastContainer } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

import Navbar from '../src/components/Navbar';

import WalletContextProvider from '../contexts/WalletContextProvider';
import TransitionContextProvider from '../contexts/TransitionContextProvider';
import Head from 'next/head';

function MyApp({ Component, pageProps }: AppProps) {
    return (
        <>
            <Head>
                <title>fr.market</title>
                <link
                    rel="icon"
                    href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🔨</text></svg>"
                />
            </Head>
            <WalletContextProvider>
                <div className="max-w-6xl mx-auto p-4">
                    <Navbar />
                    <ToastContainer />
                    <TransitionContextProvider>
                        <Component {...pageProps} />
                    </TransitionContextProvider>
                </div>
            </WalletContextProvider>
        </>
    );
}

export default MyApp;
