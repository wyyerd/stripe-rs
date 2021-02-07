export STRIPE_MOCK_VERSION=0.90.0

if [[ "$OSTYPE" == "darwin"* ]]; then
  export TARGET=darwin_amd64
else
  export TARGET=linux_amd64
fi

if [ ! -d "stripe-mock/stripe-mock_${STRIPE_MOCK_VERSION}" ]; then
  mkdir -p stripe-mock/stripe-mock_${STRIPE_MOCK_VERSION}/
  curl -L "https://github.com/stripe/stripe-mock/releases/download/v${STRIPE_MOCK_VERSION}/stripe-mock_${STRIPE_MOCK_VERSION}_${TARGET}.tar.gz" -o "stripe-mock/stripe-mock_${STRIPE_MOCK_VERSION}_${TARGET}.tar.gz"
  tar -zxf "stripe-mock/stripe-mock_${STRIPE_MOCK_VERSION}_${TARGET}.tar.gz" -C "stripe-mock/stripe-mock_${STRIPE_MOCK_VERSION}/"
fi

