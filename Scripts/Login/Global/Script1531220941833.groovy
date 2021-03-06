import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://uk.nordtek.dev.erise.hu/')

WebUI.click(findTestObject('Page_Nordtek/a_Sign In'))

WebUI.setText(findTestObject('Page_Customer Login/input_loginusername'), 'qweqwe')

WebUI.click(findTestObject('Page_Customer Login/button_Sign In'))

WebUI.waitForElementPresent(findTestObject('Page_Customer Login/div_Please enter a valid email'), 10)

WebUI.setText(findTestObject('Page_Customer Login/input_loginusername'), 'muller.laszlo@erise.hu')

WebUI.setText(findTestObject('Page_Customer Login/input_loginpassword'), 'qwe')

WebUI.click(findTestObject('Page_Customer Login/button_Sign In'))

WebUI.waitForElementPresent(findTestObject('Page_Customer Login/div_Invalid login or password.'), 10)

WebUI.setText(findTestObject('Page_Customer Login/input_loginpassword'), 'Lacika88')

WebUI.click(findTestObject('Page_Customer Login/button_Sign In'))

WebUI.waitForElementPresent(findTestObject('Page_Nordtek/a_Sign Out'), 10)

WebUI.closeBrowser()

