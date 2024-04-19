import { useInternetIdentity } from "ic-use-internet-identity";

export default function Principal({ principal }: { principal?: string }) {
  const { identity } = useInternetIdentity();

  if (!identity) return null;

  return (
    <div className="flex flex-col flex-wrap items-center w-full gap-5 md:gap-0 md:flex-row">
      Your principal is:
      {identity.getPrincipal().toString()}
    </div>
  );
}
