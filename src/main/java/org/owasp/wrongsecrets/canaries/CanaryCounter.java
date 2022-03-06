package org.owasp.wrongsecrets.canaries;

public interface CanaryCounter{

    public abstract void upCallBackCounter();
    public abstract int getTotalCount();
    public abstract void setLastCanaryToken(String tokenContent);
    public abstract String getLastToken();

}
