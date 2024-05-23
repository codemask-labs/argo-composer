dry_run="default"
pre_release="default"

if [ "true" = "true" ]; then
dry_run="--dry-run"
fi

if [ "beta" != "main" ]; then
pre_release="--preRelease=${{ inputs.release-branch }}"
fi

echo "Dry Run: $dry_run"
echo "Pre Release: $pre_release"

echo "release-it --ci $pre_release $dry_run"
