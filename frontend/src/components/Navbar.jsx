import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import Image from 'next/image';
import Link from 'next/link';
import { Music } from 'lucide-react';
import { useEffect, useState } from 'react';

const Navbar = () => {
    const [isMounted, setIsMounted] = useState(false);

    useEffect(() => {
        setIsMounted(true);
    }, []);

    return (
        <header className="flex justify-between items-center mb-8">
            <Link href="/">
                <Image
                    src="/title.svg"
                    alt="wkly.music"
                    className="h-6 sm:h-10"
                    width={120}
                    height={120}
                />
            </Link>
            <div className="flex items-center">
                <button className="sm:mr-4 md:mr-4 w-6 h-6 flex items-center justify-center rounded-full border border-black border-1 bg-white hover:bg-gray-100 text-black focus:outline-none">
                    ?
                </button>
                <a
                    href="https://charts.youtube.com/charts/TrendingVideos/us/RightNow"
                    className="mr-4 w-6 h-6 flex items-center justify-center rounded-full bg-white hover:bg-gray-100 text-black focus:outline-none"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    <Music size={24} />
                </a>
                <div className="mr-2">
                    {isMounted && ( // Render only after the component has mounted
                        <WalletMultiButton 
                            className="!bg-black hover:!bg-gray-800 transition-all duration-200 !rounded-lg"
                        />
                    )}
                </div>
            </div>
        </header>
    );
};

export default Navbar;
