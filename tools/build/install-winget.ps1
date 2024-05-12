mkdir ./msft
cd ./msft

$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri https://www.nuget.org/api/v2/package/Microsoft.UI.Xaml/2.8.6 -OutFile .\microsoft.ui.xaml.2.8.6.zip
Expand-Archive .\microsoft.ui.xaml.2.8.6.zip
Invoke-WebRequest -Uri https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx -OutFile Microsoft.VCLibs.x64.14.00.Desktop.appx

$gitHubReleasesResponse = Invoke-RestMethod https://api.github.com/repos/microsoft/winget-cli/releases/latest
$wingetReleaseAssets = $gitHubReleasesResponse.assets
$latestWingetMsixBundleUri = $wingetReleaseAssets.browser_download_url | Where-Object {$_.EndsWith(".msixbundle")}
$latestWingetLicenseXmlUri = $wingetReleaseAssets.browser_download_url | Where-Object {$_.EndsWith("License1.xml")}
Write-Host "latest_winget_msix_bundle_uri=$latestWingetMsixBundleUri"
Write-Host "latest_winget_license_xml_uri=$latestWingetLicenseXmlUri"
Write-Output "latest_winget_msix_bundle_uri=$latestWingetMsixBundleUri" >> $Env:GITHUB_OUTPUT
Write-Output "latest_winget_license_xml_uri=$latestWingetLicenseXmlUri" >> $Env:GITHUB_OUTPUT

Invoke-WebRequest -Uri "$latestWingetMsixBundleUri" -OutFile "./winget.msixbundle"
Invoke-WebRequest -Uri "$latestWingetLicenseXmlUri" -OutFile "./winget_License1.xml"

Add-AppxPackage .\microsoft.ui.xaml.2.8.6\tools\AppX\x64\Release\Microsoft.UI.Xaml.2.8.appx
Add-AppxPackage Microsoft.VCLibs.x64.14.00.Desktop.appx
Add-AppxProvisionedPackage -Online -PackagePath ./winget.msixbundle -LicensePath ./winget_License1.xml

cd ..
Remove-Item -Recurse -Force .\msft