try {
    $obj = Get-CimInstance -Namespace "root\WMI" -ClassName "LENOVO_GAMEZONE_DATA"
    if ($obj) {
        $obj | Out-String | Out-File "D:\Projects\LOQ-24-zone-RGB-controller\wmi_test.txt"
        
        # Test calling SetLightControlOwner
        try {
            $res = $obj | Invoke-CimMethod -MethodName "SetLightControlOwner" -Arguments @{ Data = [int]1 }
            "SetLightControlOwner Success: $res" | Out-File -Append "D:\Projects\LOQ-24-zone-RGB-controller\wmi_test.txt"
        } catch {
            "SetLightControlOwner Method call failed: $_" | Out-File -Append "D:\Projects\LOQ-24-zone-RGB-controller\wmi_test.txt"
        }
    } else {
        "LENOVO_GAMEZONE_DATA instance not found" | Out-File "D:\Projects\LOQ-24-zone-RGB-controller\wmi_test.txt"
    }
} catch {
    "Failed to query LENOVO_GAMEZONE_DATA: $_" | Out-File "D:\Projects\LOQ-24-zone-RGB-controller\wmi_test.txt"
}
